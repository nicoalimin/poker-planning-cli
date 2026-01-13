use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    widgets::{Block, Borders, Paragraph, Clear, BorderType},
    style::{Color, Style, Modifier, Stylize},
    text::{Line, Span},
    symbols,
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
                Constraint::Length(3), // Top Bar (Status)
                Constraint::Min(0),    // Main Game Area (2D Map)
                Constraint::Length(5), // Bottom (Controls/Stats)
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

        // 2. Main Area (2D Map)
        let map_rect = chunks[1];
        let map_block = Block::default().borders(Borders::ALL).title("Room");
        f.render_widget(map_block, map_rect);
        
        let inner_rect = map_rect.inner(ratatui::layout::Margin { vertical: 1, horizontal: 1 });
        
        // Draw Zones
        let scale_x = 3;
        let scale_y = 2;
        
        let zones = crate::zones::calculate_zones(&state.config);
        for zone in zones {
             // Scale zone coordinates to match player movement scale
             // Assume zone.x/y are in "game units" same as player.x/y
             // But existing zone logic used exact chars.
             // We need to align them.
             // Let's assume Zone structs are in "Grid Units" now.
             // We need to update zones.rs to output Grid Units, or scale here.
             
             // If zones.rs returns "char" coords, we should convert or just render.
             // Let's assume zones.rs returns Grid Units (small numbers like 2, 5, etc)
             // and we scale them here.
             
             let zone_rect = Rect {
                 x: inner_rect.x + (zone.x * scale_x),
                 y: inner_rect.y + (zone.y * scale_y),
                 width: zone.width * (scale_x / 2), // Adjust width scaling? Or just fixed size?
                 height: zone.height * scale_y / 2, // Adjust?
             };
             // Wait, current zones.rs returns "x: start_x + (i * spacing)".
             // start_x=2, spacing=10.
             // If these are grid units, 10 is HUGE if we mult by 4 (40 chars).
             // That's actually probably what we want for "Big enough visible box".
             
             let zone_rect = Rect {
                 x: inner_rect.x + (zone.x * scale_x),
                 y: inner_rect.y + (zone.y * scale_y),
                 width: zone.width * scale_x, 
                 height: zone.height * scale_y, 
             };
             // Ensure it fits
             if zone_rect.right() <= inner_rect.right() && zone_rect.bottom() <= inner_rect.bottom() {
                 let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(format!(" {} ", zone.value));
                 f.render_widget(block, zone_rect);
             }
        }

        // Draw players
        for player in state.players.values() {
            let (x, y) = player.position;
            // Simple mapping: 1 unit = 2 chars horizontally, 1 char vertically?
            // To prevent out of bounds, clean up coordinates logic
             if true { // Simplified check, doing per-rect check later
                 let symbol = match player.symbol {
                     common::AvatarSymbol::Human => "웃",
                     common::AvatarSymbol::Alien => "👽",
                     common::AvatarSymbol::Robot => "🤖",
                     common::AvatarSymbol::Ghost => "👻",
                 };
                 let color = match player.color {
                     common::AvatarColor::Red => Color::Red,
                     common::AvatarColor::Green => Color::Green,
                     common::AvatarColor::Blue => Color::Blue,
                     common::AvatarColor::Yellow => Color::Yellow,
                     common::AvatarColor::Magenta => Color::Magenta,
                     common::AvatarColor::Cyan => Color::Cyan,
                 };
                 
                 // Render as a box
                 // Scale factor check: map (x,y) to screen space?
                 // For now, let's keep 1:1 coordinate but draw a bigger box centered-ish?
                 // OR scale coordinates: screen_x = x * 4, screen_y = y * 2
                 // OR scale coordinates: screen_x = x * 4, screen_y = y * 2
                 // This makes the world "bigger".
                 let scale_x = 3;
                 let scale_y = 2;
                 
                 let screen_x = inner_rect.x + (x * scale_x);
                 let screen_y = inner_rect.y + (y * scale_y);

                 let area = Rect {
                     x: screen_x,
                     y: screen_y,
                     width: scale_x,
                     height: scale_y,
                 };
                 
                 // Boundary check with scaled coords
                 if area.right() <= inner_rect.right() && area.bottom() <= inner_rect.bottom() {
                    // Highlight self with Border
                    let block_type = if Some(player.id) == app.self_id {
                        Borders::ALL
                    } else {
                        Borders::NONE
                    };
                    
                     let paragraph = Paragraph::new(symbol)
                        .block(Block::default().borders(block_type))
                        .style(Style::default().bg(color).fg(Color::Black).add_modifier(Modifier::BOLD))
                        .alignment(Alignment::Center);
                        
                     // Visibility logic
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
                        // Name label above?
                        if screen_y > inner_rect.y {
                             let name_area = Rect { x: screen_x, y: screen_y - 1, width: (player.name.len() as u16).max(3), height: 1 };
                             if name_area.right() <= inner_rect.right() {
                                f.render_widget(Paragraph::new(player.name.as_str()).style(Style::default().fg(Color::White)), name_area);
                             }
                        }
                     }
                 }
             }
        }
        
        // 3. Stats / Instructions
        
        let help_text = match state.phase {
            Phase::Voting { .. } => "VOTING ACTIVE! Move to area or Press 1, 2, 3, 5, 8 to vote.",
            Phase::Revealed => {
                 // Calculate stats
                 // Simple text for now
                 "VOTING CLOSED. Check stats."
            },
            Phase::Idle => "Waiting for Scrum Master to start vote. (SM Keys: 's' start, 'r' reveal, '0' reset)",
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
                 stats_text = format!("Count: {} | Avg: {:.1} | Min: {} | Max: {}", count, avg, min, max);
             } else {
                 stats_text = "No votes cast.".to_string();
             }
        }

        let bottom_text = format!("{}\n{}", help_text, stats_text);
        
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(chunks[2]);
            
        let bottom = Paragraph::new(bottom_text)
            .block(Block::default().borders(Borders::ALL).title("Info"));
        f.render_widget(bottom, bottom_chunks[0]);
        
        let logs_text = app.logs.join("\n");
        let logs = Paragraph::new(logs_text)
            .block(Block::default().borders(Borders::ALL).title("Logs"));
        f.render_widget(logs, bottom_chunks[1]);
    }
}
