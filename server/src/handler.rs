use common::{ClientPayload, ServerPayload, GameState, Phase, VotingConfig, AdminCommand, Player, Role, current_time_unix, Ticket};
use crate::state::{SharedState, Tx};
use uuid::Uuid;

pub async fn handle_message(player_id: Uuid, payload: ClientPayload, state: &SharedState, tx: &Tx) {
    let mut broadcast_needed = false;
    
    // Lock state only for short duration
    {
        let mut locked_state = state.lock().unwrap();
        
        match payload {
            ClientPayload::Login { name, role, color, symbol } => {
                // Add player
                let player = Player {
                    id: player_id,
                    name,
                    role,
                    position: (2, 2), // Top-left start pos
                    color,
                    symbol,
                    confirmed: false,
                };
                locked_state.game_state.players.insert(player_id, player);
                locked_state.add_client(player_id, tx.clone());
                
                // Send Welcome
                let _ = tx.send(ServerPayload::Welcome {
                    self_id: player_id,
                    state: locked_state.game_state.clone(),
                });
                
                broadcast_needed = true;
            },
            ClientPayload::Move { x, y } => {
                if let Some(player) = locked_state.game_state.players.get_mut(&player_id) {
                    player.position = (x, y);
                    
                    // Check auto-reveal condition: Everyone in area?
                    // Simplified: if voting, just update pos. Visibility handled by client logic effectively 
                    // (although we decided server sends everything).
                    
                    broadcast_needed = true;
                }
            },
            ClientPayload::Vote { value } => {
                // Check if voting is active
                if let Phase::Voting { .. } = locked_state.game_state.phase {
                     locked_state.game_state.votes.insert(player_id, value);
                     broadcast_needed = true;
                }
            },
            ClientPayload::VoteConfirm { confirmed } => {
                if let Some(player) = locked_state.game_state.players.get_mut(&player_id) {
                    player.confirmed = confirmed;
                    broadcast_needed = true;
                }
            },
            ClientPayload::Admin(cmd) => {
                // Verify admin (optional, for now trust role)
                let is_admin = locked_state.game_state.players.get(&player_id)
                    .map(|p| p.role == Role::ScrumMaster)
                    .unwrap_or(false);
                
                if is_admin {
                    match cmd {
                        AdminCommand::StartVote { ticket, timeout } => {
                            locked_state.game_state.phase = Phase::Voting {
                                start_time_unix: current_time_unix(),
                                duration_secs: timeout,
                            };
                            locked_state.game_state.current_ticket = ticket;
                            locked_state.game_state.votes.clear();
                            for p in locked_state.game_state.players.values_mut() {
                                p.confirmed = false;
                            }
                            broadcast_needed = true;
                        },
                        AdminCommand::Reveal => {
                            locked_state.game_state.phase = Phase::Revealed;
                            broadcast_needed = true;
                        },
                        AdminCommand::Reset => {
                             locked_state.game_state.phase = Phase::Idle;
                             locked_state.game_state.votes.clear();
                             locked_state.game_state.current_ticket = None;
                             // Reset positions?
                             for p in locked_state.game_state.players.values_mut() {
                                 p.position = (10, 10);
                             }
                             broadcast_needed = true;
                        },
                        AdminCommand::Kick { player_id: target } => {
                            locked_state.remove_client(target);
                            broadcast_needed = true;
                        },
                        AdminCommand::UpdateConfig(cfg) => {
                            locked_state.game_state.config = cfg.clone();
                            // Save to disk
                            let _ = std::fs::write("config.json", serde_json::to_string_pretty(&cfg).unwrap());
                            broadcast_needed = true;
                        }
                    }
                }
            }
        }
        
        if broadcast_needed {
            locked_state.broadcast_state();
        }
    }
}
