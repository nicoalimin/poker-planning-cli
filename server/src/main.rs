use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};
use tokio::sync::mpsc;
use futures::SinkExt;
use futures::StreamExt;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::net::SocketAddr;

use common::{ClientPayload, ServerPayload, Player, Role, Phase, GameState, VotingConfig};
mod state;
mod handler;
use state::{ServerState, SharedState};

// ... imports
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

fn save_config(cfg: &VotingConfig) {
    let _ = fs::write(CONFIG_PATH, serde_json::to_string_pretty(cfg).unwrap());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    let mut state_val = ServerState::new();
    state_val.game_state.config = load_config();
    let state = Arc::new(Mutex::new(state_val));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr, state).await {
                eprintln!("Error handling connection from {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, state: SharedState) -> Result<(), Box<dyn std::error::Error>> {
    let mut framed = Framed::new(stream, LinesCodec::new());
    let (tx, mut rx) = mpsc::unbounded_channel();
    
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
    let mut outgoing_rx = rx;
    let (mut stream_tx, mut stream_rx) = framed.split();
    
    // Spawn a task to forward messages from the channel to the TCP stream
    let forward_task = tokio::spawn(async move {
        while let Some(msg) = outgoing_rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if let Err(e) = stream_tx.send(json).await {
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
                // println!("Received: {}", clean_line); // Debug
                if let Ok(payload) = serde_json::from_str::<ClientPayload>(clean_line) {
                    handler::handle_message(player_id, payload, &state, &tx).await;
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
    
    // We don't need to abort forward_task explicitly as the channel sender (tx) will be dropped, 
    // causing recv to return None, terminating the loop.
    
    Ok(())
}
