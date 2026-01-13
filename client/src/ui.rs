use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    widgets::{Block, Borders, Paragraph},
    style::{Color, Style, Modifier},
    text::{Line, Span},
};
use crate::app::{App, CurrentScreen};
use common::{Role, Phase};
// use uuid::Uuid; // Unused

pub fn ui(f: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Login => draw_login(f, app),
        CurrentScreen::Main => draw_main(f, app),
    }
}

fn draw_login(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(3), // Name Input
                Constraint::Length(3), // Role
                Constraint::Length(3), // Color
                Constraint::Length(3), // Symbol
                Constraint::Length(3), // Info
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("Poker Planning CLI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let input = Paragraph::new(app.name_input.value())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Enter Name"));
    f.render_widget(input, chunks[1]);
    
    // Set Cursor
    f.set_cursor_position(
        (chunks[1].x + 1 + app.name_input.visual_cursor() as u16, chunks[1].y + 1)
    );
    
    let role_text = format!("Role: {:?} (Press TAB)", app.role_input);
    let role = Paragraph::new(role_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(role, chunks[2]);
    
    let color_text = format!("Color: {:?} (Press F1)", app.color_input);
    let color_p = Paragraph::new(color_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(color_p, chunks[3]);

    let symbol_text = format!("Symbol: {:?} (Press F2)", app.symbol_input);
    let symbol_p = Paragraph::new(symbol_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(symbol_p, chunks[4]);

    let info = Paragraph::new("Press ENTER to connect, ESC to quit");
    f.render_widget(info, chunks[5]);
}

fn draw_main(f: &mut Frame, app: &App) {
    if let Some(state) = &app.game_state {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Top Bar (Status)
                Constraint::Min(10),    // Middle (Map - Full Width)
                Constraint::Length(12), // Bottom (Players + Help)
            ].as_ref())
            .split(f.area());
            
        // 1. Status Bar
        let phase_str = match state.phase {
            Phase::Idle => "IDLE",
            Phase::Voting { .. } => "VOTING",
            Phase::Revealed => "REVEALED",
        };
        let status_text = format!("Phase: {} | Ticket: {:?} | Players: {}", 
            phase_str, 
            state.current_ticket.as_ref().map(|t| t.title.as_str()).unwrap_or("None"),
            state.players.len()
        );
        let status_bar = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::Green));
        f.render_widget(status_bar, chunks[0]);

        // 2. Middle Area: Map (Full Width)
        let map_rect = chunks[1];
        let map_block = Block::default().borders(Borders::ALL).title("Room");
        f.render_widget(map_block, map_rect);
        
        let inner_rect = map_rect.inner(ratatui::layout::Margin { vertical: 1, horizontal: 1 });
        
        // Render Zones (Simplified loop based on previous reads)
        // Calculate pulse for unconfirmed zones
        // SystemTime might be jittery, frame-based is better if we had it in App, but simple time works.
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let pulse = (timestamp / 500) % 2 == 0; // Toggle every 500ms
        
        let zones = crate::zones::calculate_zones(&state.config);
        for zone in zones {
             let zone_rect = Rect {
                 x: inner_rect.x + (zone.x * 2), // Scale x 2
                 y: inner_rect.y + (zone.y * 1), // Scale y 1
                 width: zone.width * 2, 
                 height: zone.height * 1, 
             };
             
             if zone_rect.right() <= inner_rect.right() && zone_rect.bottom() <= inner_rect.bottom() {
                 // Check if SELF player is in this zone
                 let self_player = app.self_id.and_then(|id| state.players.get(&id));
                 let mut is_in_zone = false;
                 let mut is_confirmed = false;
                 
                 if let Some(p) = self_player {
                     if p.position.0 >= zone.x && p.position.0 < zone.x + zone.width &&
                        p.position.1 >= zone.y && p.position.1 < zone.y + zone.height {
                            is_in_zone = true;
                            is_confirmed = p.confirmed;
                        }
                 }
                 
                 let border_style = if is_in_zone {
                     if is_confirmed {
                         // Persistent Glow (Gold)
                         Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                     } else {
                         // Sporadic Glow (Pulse)
                         if pulse {
                              Style::default().fg(Color::Cyan)
                         } else {
                              Style::default().fg(Color::DarkGray)
                         }
                     }
                 } else {
                     Style::default().fg(Color::DarkGray)
                 };
                 
                 let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(format!(" {} ", zone.value));
                 f.render_widget(block, zone_rect);
             }
        }

        // Draw players
        for player in state.players.values() {
            let (x, y) = player.position;
            
             let symbol = match player.symbol {
                 common::AvatarSymbol::Human => "웃",
                 common::AvatarSymbol::Alien => "👽",
                 common::AvatarSymbol::Robot => "🤖",
                 common::AvatarSymbol::Ghost => "👻",
                 common::AvatarSymbol::Cat => "🐱",
                 common::AvatarSymbol::Dog => "🐶",
                 common::AvatarSymbol::Bird => "🐦",
                 common::AvatarSymbol::Fish => "🐠",
                 common::AvatarSymbol::Tree => "🌲",
                 common::AvatarSymbol::Flower => "🌺",
                 common::AvatarSymbol::Star => "⭐",
                 common::AvatarSymbol::Moon => "🌙",
                 common::AvatarSymbol::Sun => "☀️",
                 common::AvatarSymbol::Heart => "❤️",
                 common::AvatarSymbol::Skull => "💀",
                 common::AvatarSymbol::Smile => "😊",
                 common::AvatarSymbol::Zap => "⚡",
                 common::AvatarSymbol::Anchor => "⚓",
                 common::AvatarSymbol::Music => "🎵",
                 common::AvatarSymbol::Globe => "🌍",
             };
             let color = match player.color {
                 common::AvatarColor::Red => Color::Red,
                 common::AvatarColor::Green => Color::Green,
                 common::AvatarColor::Blue => Color::Blue,
                 common::AvatarColor::Yellow => Color::Yellow,
                 common::AvatarColor::Magenta => Color::Magenta,
                 common::AvatarColor::Cyan => Color::Cyan,
                 common::AvatarColor::Orange => Color::Rgb(255, 165, 0),
                 common::AvatarColor::Pink => Color::Rgb(255, 192, 203),
                 common::AvatarColor::Purple => Color::Rgb(128, 0, 128),
                 common::AvatarColor::Mint => Color::Rgb(189, 252, 201),
                 common::AvatarColor::Gold => Color::Rgb(255, 215, 0),
                 common::AvatarColor::Silver => Color::Rgb(192, 192, 192),
                 common::AvatarColor::Bronze => Color::Rgb(205, 127, 50),
                 common::AvatarColor::Lime => Color::Rgb(50, 205, 50),
                 common::AvatarColor::Teal => Color::Rgb(0, 128, 128),
                 common::AvatarColor::Indigo => Color::Rgb(75, 0, 130),
                 common::AvatarColor::Violet => Color::Rgb(238, 130, 238),
                 common::AvatarColor::Coral => Color::Rgb(255, 127, 80),
                 common::AvatarColor::Crimson => Color::Rgb(220, 20, 60),
                 common::AvatarColor::White => Color::White,
             };
                 
             let scale_x = 2;
             let scale_y = 1;
             
             let screen_x = inner_rect.x + (x * scale_x);
             let screen_y = inner_rect.y + (y * scale_y);

             let area = Rect {
                 x: screen_x,
                 y: screen_y,
                 width: scale_x,
                 height: scale_y,
             };
             
             if area.right() <= inner_rect.right() && area.bottom() <= inner_rect.bottom() {
                let block_type = if Some(player.id) == app.self_id {
                    Borders::ALL
                } else {
                    Borders::NONE
                };
                
                 let paragraph = Paragraph::new(symbol)
                    .block(Block::default().borders(block_type))
                    .style(Style::default().bg(color).fg(Color::Black).add_modifier(Modifier::BOLD))
                    .alignment(Alignment::Center);
                    
                 let can_see = match app.role_input {
                     Role::ScrumMaster => true,
                     _ => {
                         if let Phase::Voting { .. } = state.phase {
                             Some(player.id) == app.self_id
                         } else {
                             true
                         }
                     }
                 };

                 if can_see {
                    f.render_widget(paragraph, area);
                    if screen_y > inner_rect.y {
                         let name_area = Rect { x: screen_x, y: screen_y - 1, width: (player.name.len() as u16).max(3), height: 1 };
                         if name_area.right() <= inner_rect.right() {
                            f.render_widget(Paragraph::new(player.name.as_str()).style(Style::default().fg(Color::White)), name_area);
                         }
                    }
                 }
             }
        }
        
        // 3. Bottom Section (Players + Help + Info)
        let bottom_chunks = Layout::default()
             .direction(Direction::Horizontal)
             .constraints([
                 Constraint::Percentage(33), // Connected Players (Left)
                 Constraint::Percentage(33), // Help (Middle)
                 Constraint::Percentage(34), // Info (Right)
             ].as_ref())
             .split(chunks[2]);

        // Column 1: Connected Players (Sorted)
        let mut sorted_players: Vec<_> = state.players.values().collect();
        sorted_players.sort_by_key(|p| p.name.to_lowercase());

        let mut player_lines = Vec::new();
        for p in sorted_players {
             let p_symbol = match p.symbol {
                 common::AvatarSymbol::Human => "웃",
                 common::AvatarSymbol::Alien => "👽",
                 common::AvatarSymbol::Robot => "🤖",
                 common::AvatarSymbol::Ghost => "👻",
                 common::AvatarSymbol::Cat => "🐱",
                 common::AvatarSymbol::Dog => "🐶",
                 common::AvatarSymbol::Bird => "🐦",
                 common::AvatarSymbol::Fish => "🐠",
                 common::AvatarSymbol::Tree => "🌲",
                 common::AvatarSymbol::Flower => "🌺",
                 common::AvatarSymbol::Star => "⭐",
                 common::AvatarSymbol::Moon => "🌙",
                 common::AvatarSymbol::Sun => "☀️",
                 common::AvatarSymbol::Heart => "❤️",
                 common::AvatarSymbol::Skull => "💀",
                 common::AvatarSymbol::Smile => "😊",
                 common::AvatarSymbol::Zap => "⚡",
                 common::AvatarSymbol::Anchor => "⚓",
                 common::AvatarSymbol::Music => "🎵",
                 common::AvatarSymbol::Globe => "🌍",
             };
             
             let p_color = match p.color {
                    common::AvatarColor::Red => Color::Red,
                    common::AvatarColor::Green => Color::Green,
                    common::AvatarColor::Blue => Color::Blue,
                    common::AvatarColor::Yellow => Color::Yellow,
                    common::AvatarColor::Magenta => Color::Magenta,
                    common::AvatarColor::Cyan => Color::Cyan,
                    common::AvatarColor::Orange => Color::Rgb(255, 165, 0),
                    common::AvatarColor::Pink => Color::Rgb(255, 192, 203),
                    common::AvatarColor::Purple => Color::Rgb(128, 0, 128),
                    common::AvatarColor::Mint => Color::Rgb(189, 252, 201),
                    common::AvatarColor::Gold => Color::Rgb(255, 215, 0),
                    common::AvatarColor::Silver => Color::Rgb(192, 192, 192),
                    common::AvatarColor::Bronze => Color::Rgb(205, 127, 50),
                    common::AvatarColor::Lime => Color::Rgb(50, 205, 50),
                    common::AvatarColor::Teal => Color::Rgb(0, 128, 128),
                    common::AvatarColor::Indigo => Color::Rgb(75, 0, 130),
                    common::AvatarColor::Violet => Color::Rgb(238, 130, 238),
                    common::AvatarColor::Coral => Color::Rgb(255, 127, 80),
                    common::AvatarColor::Crimson => Color::Rgb(220, 20, 60),
                    common::AvatarColor::White => Color::White,
             };

             let name_style = if p.confirmed {
                 Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
             } else {
                 Style::default()
             };

            player_lines.push(Line::from(vec![
                Span::styled(format!("{} ", p_symbol), Style::default().fg(p_color)),
                Span::styled(format!("{} ({:?})", p.name, p.role), name_style)
            ]));
        }
        let player_list = Paragraph::new(player_lines)
           .block(Block::default().borders(Borders::ALL).title("Connected Players"));
        f.render_widget(player_list, bottom_chunks[0]);
        
        // Column 3: Info (Phase & Stats)
        let help_text_bottom = match state.phase {
            Phase::Voting { .. } => "VOTING ACTIVE!\nMove to area to vote.",
            Phase::Revealed => "VOTING CLOSED.",
            Phase::Idle => "Waiting for Scrum Master.",
        };
        
        // If Revealed, show stats
        let mut stats_text = String::new();
        if let Phase::Revealed = state.phase {
             let votes: Vec<u32> = state.votes.values().filter_map(|v| *v).collect();
             if !votes.is_empty() {
                 let sum: u32 = votes.iter().sum();
                 let count = votes.len();
                 let avg = sum as f32 / count as f32;
                 let min = votes.iter().min().unwrap();
                 let max = votes.iter().max().unwrap();
                 stats_text = format!("\n\nStats:\nCount: {}\nAvg: {:.1}\nMin: {}\nMax: {}", count, avg, min, max);
             } else {
                 stats_text = "\n\nNo votes cast.".to_string();
             }
        }
        let info_content = format!("{}{}", help_text_bottom, stats_text);
        
        let info_block = Paragraph::new(info_content)
            .block(Block::default().borders(Borders::ALL).title("Info"));
        f.render_widget(info_block, bottom_chunks[1]);

        // Column 2: Help (Controls)
        let help_lines = vec![
            Line::from(Span::styled("Controls:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from("Arrows: Move"),
            Line::from("Space: Confirm Vote"),
            Line::from("R: Reveal (ScrumMaster)"),
            Line::from("S: Start/Stop (ScrumMaster)"),
            Line::from("Space: Confirm/Unconfirm"),
            Line::from("Q: Quit"),
        ];

         let help_block = Paragraph::new(help_lines)
           .block(Block::default().borders(Borders::ALL).title("Help"))
           .style(Style::default().fg(Color::Gray));
        f.render_widget(help_block, bottom_chunks[2]);
    }
}
