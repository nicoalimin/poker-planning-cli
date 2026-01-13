use std::{io, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Clear},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal,
    style::{Style, Color, Modifier},
    symbols,
    text::{Line, Span},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use common::{ClientPayload, ServerPayload, Role, Phase};
use tui_input::backend::crossterm::EventHandler;

mod app;
mod network;
mod ui; // We will implement UI in a separate file too, or keep it simple here? 
// Let's create ui.rs for the draw functions
use app::{App, CurrentScreen};
use network::Network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut network: Option<Network> = None;

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        // Poll for events. NOTE: Blocking poll is tricky with async networking.
        // We should use `crossterm::event::EventStream` or verify poll non-blocking.
        // But `terminal.draw` is fast.
        // We need to poll network AND inputs.
        
        // Handle Network
        if let Some(net) = &mut network {
            while let Ok(msg) = net.rx.try_recv() {
                // Parse server message
                 if let Ok(payload) = serde_json::from_str::<ServerPayload>(&msg) {
                     match payload {
                         ServerPayload::Welcome { self_id, state } => {
                             app.self_id = Some(self_id);
                             app.game_state = Some(state);
                             app.current_screen = CurrentScreen::Main;
                         },
                         ServerPayload::StateUpdate(state) => {
                             app.game_state = Some(state);
                         },
                         ServerPayload::Error(e) => {
                             // Show error?
                         }
                     }
                 }
            }
        }

        // Handle Input with timeout to allow loop to cycle
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match app.current_screen {
                    CurrentScreen::Login => {
                        match key.code {
                            KeyCode::Enter => {
                                // Connect
                                match Network::connect(&app.server_url).await {
                                    Ok(net) => {
                                        // Send Login
                                        let login = ClientPayload::Login {
                                            name: app.name_input.value().to_string(),
                                            role: app.role_input.clone(),
                                        };
                                        let json = serde_json::to_string(&login)?;
                                        net.tx.send(json)?;
                                        network = Some(net);
                                    },
                                    Err(e) => {
                                        // TODO: Show connection error
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Tab => {
                                // Cycle role
                                app.role_input = match app.role_input {
                                    Role::Participant => Role::ScrumMaster,
                                    Role::ScrumMaster => Role::Observer,
                                    Role::Observer => Role::Participant,
                                };
                            }
                            _ => {
                                // Input name
                                app.name_input.handle_event(&Event::Key(key));
                            }
                        }
                    },
                    CurrentScreen::Main => {
                        // Main game inputs
                         match key.code {
                            KeyCode::Esc => break,
                            KeyCode::Char('q') => break,
                            KeyCode::Left => {
                                if let Some(net) = &network {
                                    let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_x = x.saturating_sub(1);
                                    let msg = ClientPayload::Move { x: new_x, y };
                                    net.tx.send(serde_json::to_string(&msg)?)?;
                                }
                            }
                            KeyCode::Right => {
                                if let Some(net) = &network {
                                     let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_x = x + 1;
                                    let msg = ClientPayload::Move { x: new_x, y };
                                    net.tx.send(serde_json::to_string(&msg)?)?;
                                }
                            }
                             KeyCode::Up => {
                                if let Some(net) = &network {
                                     let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_y = y.saturating_sub(1);
                                    let msg = ClientPayload::Move { x, y: new_y };
                                    net.tx.send(serde_json::to_string(&msg)?)?;
                                }
                            }
                             KeyCode::Down => {
                                if let Some(net) = &network {
                                     let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_y = y + 1;
                                    let msg = ClientPayload::Move { x, y: new_y };
                                    net.tx.send(serde_json::to_string(&msg)?)?;
                                }
                            }
                            // Voting hotkeys (temporary)
                            KeyCode::Char('1') => send_vote(&network, Some(1)),
                            KeyCode::Char('2') => send_vote(&network, Some(2)),
                            KeyCode::Char('3') => send_vote(&network, Some(3)),
                            KeyCode::Char('5') => send_vote(&network, Some(5)),
                            KeyCode::Char('8') => send_vote(&network, Some(8)),
                             // Admin commands
                            KeyCode::Char('s') => { // Start
                                 let cmd = common::AdminCommand::StartVote { ticket: None, timeout: Some(20) };
                                 if let Some(net) = &network {
                                     net.tx.send(serde_json::to_string(&ClientPayload::Admin(cmd))?)?;
                                 }
                            },
                             KeyCode::Char('r') => { // Reveal
                                 let cmd = common::AdminCommand::Reveal;
                                 if let Some(net) = &network {
                                     net.tx.send(serde_json::to_string(&ClientPayload::Admin(cmd))?)?;
                                 }
                            },
                             KeyCode::Char('0') => { // Reset
                                 let cmd = common::AdminCommand::Reset;
                                 if let Some(net) = &network {
                                     net.tx.send(serde_json::to_string(&ClientPayload::Admin(cmd))?)?;
                                 }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}

fn send_vote(network: &Option<Network>, val: Option<u32>) {
     if let Some(net) = network {
         let msg = ClientPayload::Vote { value: val };
         let _ = net.tx.send(serde_json::to_string(&msg).unwrap());
     }
}
