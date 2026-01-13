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
mod zones;
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
        
        // Calculate dynamic boundaries based on terminal size
        // Layout duplicate logic:
        // Top status (3) + Bottom (5) ... wait, ui.rs constraints says:
        /*
            Constraint::Length(3), // Status
            Constraint::Min(10), // Map
            Constraint::Length(5), // Bottom
        */
        // Margin 2
        
        let size = terminal.size()?;
        if size.width > 4 && size.height > 10 { // Minimal check
             let available_width = size.width.saturating_sub(4); // Margin 2 either side
             // Inner map also has margin 1 either side -> total -2
             let map_inner_width = available_width.saturating_sub(2);
             
             let available_height = size.height.saturating_sub(4); // Margin 2 top/bottom
             let map_height = available_height.saturating_sub(3 + 5); // Status + Bottom
             // Inner map margin 1 top/bottom
             let map_inner_height = map_height.saturating_sub(2);
             
             // Scale 3x2
             app.grid_width = map_inner_width / 3;
             app.grid_height = map_inner_height / 2;
        }

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
                             app.log("Connected to server".to_string());
                             app.self_id = Some(self_id);
                             app.game_state = Some(state);
                             app.current_screen = CurrentScreen::Main;
                         },
                         ServerPayload::StateUpdate(state) => {
                             // app.log("State updated".to_string()); // Too spammy?
                             app.game_state = Some(state);
                         },
                         ServerPayload::Error(e) => {
                             app.log(format!("Server Error: {}", e));
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
                                            color: app.color_input.clone(),
                                            symbol: app.symbol_input.clone(),
                                        };
                                        app.log(format!("Logging in as {} ({:?})", app.name_input.value(), app.role_input));
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
                            KeyCode::F(1) => {
                                app.color_input = match app.color_input {
                                    common::AvatarColor::Red => common::AvatarColor::Green,
                                    common::AvatarColor::Green => common::AvatarColor::Blue,
                                    common::AvatarColor::Blue => common::AvatarColor::Yellow,
                                    common::AvatarColor::Yellow => common::AvatarColor::Magenta,
                                    common::AvatarColor::Magenta => common::AvatarColor::Cyan,
                                    common::AvatarColor::Cyan => common::AvatarColor::Red,
                                };
                            }
                            KeyCode::F(2) => {
                                app.symbol_input = match app.symbol_input {
                                    common::AvatarSymbol::Human => common::AvatarSymbol::Alien,
                                    common::AvatarSymbol::Alien => common::AvatarSymbol::Robot,
                                    common::AvatarSymbol::Robot => common::AvatarSymbol::Ghost,
                                    common::AvatarSymbol::Ghost => common::AvatarSymbol::Human,
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
                            KeyCode::Esc => { app.log("Quit".to_string()); break; },
                            KeyCode::Char('q') => { app.log("Quit".to_string()); break; },
                            KeyCode::Left => {
                                app.log("Pressed Left".to_string());
                                if let Some(net) = &network {
                                    let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_x = x.saturating_sub(1);
                                    if new_x >= 0 { // Boundary Check
                                        let msg = ClientPayload::Move { x: new_x, y };
                                        net.tx.send(serde_json::to_string(&msg)?)?;
                                        check_zone_vote(new_x, y, &app.game_state, net);
                                    }
                                }
                            }
                            KeyCode::Right => {
                                app.log("Pressed Right".to_string());
                                if let Some(net) = &network {
                                     let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_x = x + 1;
                                    if new_x < app.grid_width { // Dynamic Boundary
                                        let msg = ClientPayload::Move { x: new_x, y };
                                        net.tx.send(serde_json::to_string(&msg)?)?;
                                        check_zone_vote(new_x, y, &app.game_state, net);
                                    }
                                }
                            }
                             KeyCode::Up => {
                                app.log("Pressed Up".to_string());
                                if let Some(net) = &network {
                                     let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_y = y.saturating_sub(1);
                                    if new_y >= 0 { // Boundary Check
                                        let msg = ClientPayload::Move { x, y: new_y };
                                        net.tx.send(serde_json::to_string(&msg)?)?;
                                        check_zone_vote(x, new_y, &app.game_state, net);
                                    }
                                }
                            }
                             KeyCode::Down => {
                                app.log("Pressed Down".to_string());
                                if let Some(net) = &network {
                                     let (x,y) = app.game_state.as_ref().map(|s| {
                                        s.players.get(&app.self_id.unwrap()).map(|p| p.position).unwrap_or((10,10))
                                    }).unwrap_or((10,10));
                                    
                                    let new_y = y + 1;
                                    if new_y < app.grid_height { // Dynamic Boundary
                                        let msg = ClientPayload::Move { x, y: new_y };
                                        net.tx.send(serde_json::to_string(&msg)?)?;
                                        check_zone_vote(x, new_y, &app.game_state, net);
                                    }
                                }
                            }
                            // Voting hotkeys (temporary)
                            KeyCode::Char('1') => { app.log("Voted 1".to_string()); send_vote(&network, Some(1)) },
                            KeyCode::Char('2') => { app.log("Voted 2".to_string()); send_vote(&network, Some(2)) },
                            KeyCode::Char('3') => { app.log("Voted 3".to_string()); send_vote(&network, Some(3)) },
                            KeyCode::Char('5') => { app.log("Voted 5".to_string()); send_vote(&network, Some(5)) },
                            KeyCode::Char('8') => { app.log("Voted 8".to_string()); send_vote(&network, Some(8)) },
                             // Admin commands
                            KeyCode::Char('s') => { // Start
                                 app.log("Admin: Start Vote".to_string());
                                 let cmd = common::AdminCommand::StartVote { ticket: None, timeout: Some(20) };
                                 if let Some(net) = &network {
                                     net.tx.send(serde_json::to_string(&ClientPayload::Admin(cmd))?)?;
                                 }
                            },
                             KeyCode::Char('r') => { // Reveal
                                 app.log("Admin: Reveal".to_string());
                                 let cmd = common::AdminCommand::Reveal;
                                 if let Some(net) = &network {
                                     net.tx.send(serde_json::to_string(&ClientPayload::Admin(cmd))?)?;
                                 }
                            },
                             KeyCode::Char('0') => { // Reset
                                 app.log("Admin: Reset".to_string());
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

fn check_zone_vote(x: u16, y: u16, state: &Option<common::GameState>, net: &Network) {
    if let Some(s) = state {
        let zones = crate::zones::calculate_zones(&s.config);
        for zone in zones {
             if x >= zone.x && x < zone.x + zone.width && y >= zone.y && y < zone.y + zone.height {
                 // In zone, vote!
                 let msg = ClientPayload::Vote { value: Some(zone.value) };
                 let _ = net.tx.send(serde_json::to_string(&msg).unwrap());
                 return;
             }
        }
        // If not in any zone, maybe unvote? 
        // "Reset the state... pull everyone back".
        // Let's imply leaving zone = keep vote? Or unvote?
        // Prompt: "can vote... change their vote".
        // Usually spatial means if I leave, I haven't voted.
        // But let's stick to explicit vote for now to be safe, or unvote if outside?
        // Let's Unvote if outside all zones?
        // "until every participant has answered".
        // If I walk out, did I withdraw my answer?
        // Let's implement unvote if outside all zones.
        
        // Check if we WERE in a zone (to avoid spamming unvote).
        // Too complex for stateless check.
        // Just send Unvote (None) if not in zone.
        let msg = ClientPayload::Vote { value: None };
        let _ = net.tx.send(serde_json::to_string(&msg).unwrap());
    }
}
