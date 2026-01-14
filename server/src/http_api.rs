use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tower_http::cors::{Any, CorsLayer};

use crate::state::SharedState;

// Request/Response types for the HTTP API

#[derive(Debug, Deserialize)]
pub struct StartVotingRequest {
    pub issue_number: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StartVotingResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct VoteDetail {
    pub player_name: String,
    pub vote: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct RevealVotesResponse {
    pub success: bool,
    pub issue_number: Option<String>,
    pub votes: Vec<VoteDetail>,
    pub statistics: VoteStatistics,
}

#[derive(Debug, Serialize)]
pub struct VoteStatistics {
    pub total_voters: usize,
    pub votes_cast: usize,
    pub average: Option<f64>,
    pub median: Option<f64>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub mode: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusUpdate {
    pub phase: String,
    pub issue_number: Option<String>,
    pub connected_players: Vec<ConnectedPlayer>,
    pub votes_cast: usize,
    pub total_players: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectedPlayer {
    pub name: String,
    pub has_voted: bool,
}

// Shared state for HTTP API including broadcast channel
pub struct HttpState {
    pub game_state: SharedState,
    pub status_tx: tokio::sync::broadcast::Sender<StatusUpdate>,
}

pub fn create_router(state: std::sync::Arc<HttpState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/start-voting", post(start_voting))
        .route("/api/reveal", post(reveal_votes))
        .route("/api/status", get(status_stream))
        .route("/api/status-poll", get(status_poll))
        .layer(cors)
        .with_state(state)
}

async fn start_voting(
    State(state): State<std::sync::Arc<HttpState>>,
    Json(payload): Json<StartVotingRequest>,
) -> Json<StartVotingResponse> {
    let ticket = payload.issue_number.map(|issue| common::Ticket { title: issue });

    {
        let mut locked_state = state.game_state.lock().unwrap();

        // Start voting phase
        locked_state.game_state.phase = common::Phase::Voting {
            start_time_unix: common::current_time_unix(),
            duration_secs: locked_state.game_state.config.default_timeout,
        };
        locked_state.game_state.current_ticket = ticket;
        locked_state.game_state.votes.clear();

        // Reset confirmed status for all players
        for player in locked_state.game_state.players.values_mut() {
            player.confirmed = false;
        }

        // Broadcast state update to CLI clients
        locked_state.broadcast_state();
    }

    // Send status update to SSE subscribers
    let _ = state.status_tx.send(get_current_status(&state.game_state));

    Json(StartVotingResponse {
        success: true,
        message: "Voting started".to_string(),
    })
}

async fn reveal_votes(
    State(state): State<std::sync::Arc<HttpState>>,
) -> Json<RevealVotesResponse> {
    let (votes, issue_number, statistics) = {
        let mut locked_state = state.game_state.lock().unwrap();

        // Collect vote details
        let votes: Vec<VoteDetail> = locked_state
            .game_state
            .players
            .iter()
            .map(|(id, player)| {
                let vote = locked_state.game_state.votes.get(id).copied().flatten();
                VoteDetail {
                    player_name: player.name.clone(),
                    vote,
                }
            })
            .collect();

        let issue_number = locked_state
            .game_state
            .current_ticket
            .as_ref()
            .map(|t| t.title.clone());

        // Calculate statistics
        let actual_votes: Vec<u32> = locked_state
            .game_state
            .votes
            .values()
            .filter_map(|v| *v)
            .collect();

        let statistics = calculate_statistics(&actual_votes, locked_state.game_state.players.len());

        // Set phase to Revealed
        locked_state.game_state.phase = common::Phase::Revealed;

        // Broadcast state update to CLI clients
        locked_state.broadcast_state();

        (votes, issue_number, statistics)
    };

    // Send status update to SSE subscribers
    let _ = state.status_tx.send(get_current_status(&state.game_state));

    Json(RevealVotesResponse {
        success: true,
        issue_number,
        votes,
        statistics,
    })
}

// Simple polling endpoint for Chrome extension (avoids CORS issues with SSE)
async fn status_poll(
    State(state): State<std::sync::Arc<HttpState>>,
) -> Json<StatusUpdate> {
    Json(get_current_status(&state.game_state))
}

async fn status_stream(
    State(state): State<std::sync::Arc<HttpState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Send initial status immediately
    let initial_status = get_current_status(&state.game_state);

    let rx = state.status_tx.subscribe();
    let stream = BroadcastStream::new(rx);

    // Create a stream that first sends initial status, then listens for updates
    let initial_stream = futures::stream::once(async move {
        Ok::<_, Infallible>(Event::default().json_data(initial_status).unwrap())
    });

    let update_stream = stream.filter_map(|result| match result {
        Ok(status) => Some(Ok(Event::default().json_data(status).unwrap())),
        Err(_) => None,
    });

    let combined_stream = initial_stream.chain(update_stream);

    Sse::new(combined_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

fn get_current_status(game_state: &SharedState) -> StatusUpdate {
    let locked_state = game_state.lock().unwrap();

    let phase = match &locked_state.game_state.phase {
        common::Phase::Idle => "idle".to_string(),
        common::Phase::Voting { .. } => "voting".to_string(),
        common::Phase::Revealed => "revealed".to_string(),
    };

    let issue_number = locked_state
        .game_state
        .current_ticket
        .as_ref()
        .map(|t| t.title.clone());

    let connected_players: Vec<ConnectedPlayer> = locked_state
        .game_state
        .players
        .iter()
        .map(|(id, player)| ConnectedPlayer {
            name: player.name.clone(),
            has_voted: locked_state.game_state.votes.get(id).map(|v| v.is_some()).unwrap_or(false),
        })
        .collect();

    let votes_cast = locked_state
        .game_state
        .votes
        .values()
        .filter(|v| v.is_some())
        .count();

    StatusUpdate {
        phase,
        issue_number,
        connected_players,
        votes_cast,
        total_players: locked_state.game_state.players.len(),
    }
}

fn calculate_statistics(votes: &[u32], total_players: usize) -> VoteStatistics {
    if votes.is_empty() {
        return VoteStatistics {
            total_voters: total_players,
            votes_cast: 0,
            average: None,
            median: None,
            min: None,
            max: None,
            mode: None,
        };
    }

    let mut sorted_votes = votes.to_vec();
    sorted_votes.sort();

    let sum: u32 = sorted_votes.iter().sum();
    let average = Some(sum as f64 / sorted_votes.len() as f64);

    let median = if sorted_votes.len() % 2 == 0 {
        let mid = sorted_votes.len() / 2;
        Some((sorted_votes[mid - 1] + sorted_votes[mid]) as f64 / 2.0)
    } else {
        Some(sorted_votes[sorted_votes.len() / 2] as f64)
    };

    let min = sorted_votes.first().copied();
    let max = sorted_votes.last().copied();

    // Calculate mode (most frequent value)
    let mut frequency: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    for &vote in &sorted_votes {
        *frequency.entry(vote).or_insert(0) += 1;
    }
    let mode = frequency
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(value, _)| value);

    VoteStatistics {
        total_voters: total_players,
        votes_cast: votes.len(),
        average,
        median,
        min,
        max,
        mode,
    }
}
