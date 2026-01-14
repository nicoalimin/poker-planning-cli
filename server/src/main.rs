use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};
use tokio::sync::{mpsc, broadcast};
use futures::SinkExt;
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::net::SocketAddr;

use common::{ClientPayload, Phase, VotingConfig};
mod state;
mod handler;
mod http_api;
use state::{ServerState, SharedState};
use http_api::HttpState;

use std::fs;
use std::path::Path;

const CONFIG_PATH: &str = "config.json";

fn load_config() -> VotingConfig {
    if Path::new(CONFIG_PATH).exists() {
        if let Ok(content) = fs::read_to_string(CONFIG_PATH) {
            if let Ok(cfg) = serde_json::from_str(&content) {
                return cfg;
            }
        }
    }
    let default = VotingConfig::default();
    let _ = fs::write(CONFIG_PATH, serde_json::to_string_pretty(&default).unwrap());
    default
}

#[allow(dead_code)]
fn save_config(cfg: &VotingConfig) {
    let _ = fs::write(CONFIG_PATH, serde_json::to_string_pretty(cfg).unwrap());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_addr = "0.0.0.0:8888";
    let http_addr = "0.0.0.0:8887";

    let mut state_val = ServerState::new();
    state_val.game_state.config = load_config();
    let state = Arc::new(Mutex::new(state_val));

    // Create broadcast channel for SSE status updates
    let (status_tx, _) = broadcast::channel::<http_api::StatusUpdate>(100);

    // Create HTTP state
    let http_state = Arc::new(HttpState {
        game_state: state.clone(),
        status_tx: status_tx.clone(),
    });

    // Start HTTP server
    let http_router = http_api::create_router(http_state);
    let http_listener = tokio::net::TcpListener::bind(http_addr).await?;
    println!("HTTP API listening on {}", http_addr);

    tokio::spawn(async move {
        axum::serve(http_listener, http_router).await.unwrap();
    });

    // Start TCP server for CLI clients
    let listener = TcpListener::bind(tcp_addr).await?;
    println!("TCP server listening on {}", tcp_addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = state.clone();
        let status_tx = status_tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, state, status_tx).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    state: SharedState,
    status_tx: broadcast::Sender<http_api::StatusUpdate>,
) -> Result<(), Box<dyn std::error::Error>> {
    let framed = Framed::new(stream, LinesCodec::new());
    let (tx, rx) = mpsc::unbounded_channel();
    
    // We need to store the tx in the state, but we don't know the Player ID yet.
    // Flow: 
    // 1. Client connects.
    // 2. Client sends Login.
    // 3. Server adds to state.
    // 4. Server sends Welcome + State.
    
    println!("New connection from {}", addr);

    // Wait for Login
    let player_id = Uuid::new_v4();
    
    // Create a loop to handle outgoing messages (from other parts of the system to this client)
    let outgoing_rx = rx;
    let (mut stream_tx, mut stream_rx) = framed.split();
    
    // Spawn a task to forward messages from the channel to the TCP stream
    let _forward_task = tokio::spawn(async move {
        let mut outgoing_rx = outgoing_rx;
        while let Some(msg) = outgoing_rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if stream_tx.send(json).await.is_err() {
                // Client disconnected or error
                println!("Client {} send error (disconnected?)", addr);
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(result) = stream_rx.next().await {
        match result {
            Ok(line) => {
                let clean_line = line.trim();
                if let Ok(payload) = serde_json::from_str::<ClientPayload>(clean_line) {
                    handler::handle_message(player_id, payload, &state, &tx).await;
                    // Broadcast status update to SSE subscribers
                    let _ = status_tx.send(get_current_status(&state));
                } else {
                    eprintln!("Failed to parse: {}", clean_line);
                }
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
                break;
            }
        }
    }

    // Cleanup
    {
        println!("Client {} cleaning up", addr);
        let mut locked_state = state.lock().unwrap();
        locked_state.remove_client(player_id);
        locked_state.broadcast_state();
    }
    
    // Broadcast status update after client disconnect
    let _ = status_tx.send(get_current_status(&state));
    
    Ok(())
}

fn get_current_status(game_state: &SharedState) -> http_api::StatusUpdate {
    let locked_state = game_state.lock().unwrap();

    let phase = match &locked_state.game_state.phase {
        Phase::Idle => "idle".to_string(),
        Phase::Voting { .. } => "voting".to_string(),
        Phase::Revealed => "revealed".to_string(),
    };

    let issue_number = locked_state
        .game_state
        .current_ticket
        .as_ref()
        .map(|t| t.title.clone());

    let connected_players: Vec<http_api::ConnectedPlayer> = locked_state
        .game_state
        .players
        .iter()
        .map(|(_, player)| http_api::ConnectedPlayer {
            name: player.name.clone(),
            has_voted: player.confirmed,
        })
        .collect();

    let votes_cast = locked_state
        .game_state
        .votes
        .values()
        .filter(|v| v.is_some())
        .count();

    http_api::StatusUpdate {
        phase,
        issue_number,
        connected_players,
        votes_cast,
        total_players: locked_state.game_state.players.len(),
    }
}
