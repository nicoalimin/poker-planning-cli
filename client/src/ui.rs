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
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
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
    
    let role_text = format!("Role: {:?} (Press TAB to cycle)", app.role_input);
    let role = Paragraph::new(role_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(role, chunks[2]);
    
    let info = Paragraph::new("Press ENTER to connect, ESC to quit");
    f.render_widget(info, chunks[3]);
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
        
        // Draw players
        for player in state.players.values() {
            let (x, y) = player.position;
            // Simple mapping: 1 unit = 2 chars horizontally, 1 char vertically?
            // To prevent out of bounds, clean up coordinates logic
             if x < inner_rect.width && y < inner_rect.height {
                 let symbol = "웃";
                 let color = match player.role {
                     Role::ScrumMaster => Color::Magenta,
                     _ => Color::White,
                 };
                 // Highlight self
                 let style = if Some(player.id) == app.self_id {
                     Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                 } else {
                     Style::default().fg(color)
                 };
                 
                 // Render at position
                 let area = Rect {
                     x: inner_rect.x + x,
                     y: inner_rect.y + y,
                     width: 1,
                     height: 1,
                 };
                 // Check visibility rules here again if we want client-side hiding
                 // If voting, hide others
                 let visible = if let Phase::Voting { .. } = state.phase {
                     Some(player.id) == app.self_id || player.role == Role::ScrumMaster // Maybe SM sees all? Rules say "Scrum master can also reset... Other participants are only revealed..."
                     // Lets follow "participant cannot see other participants"
                     // So if I am Participant, I only see myself.
                     // If I am SM, maybe I see everyone?
                 } else {
                     true
                 };

                 // Actually, let's implement the visibility rule:
                 // "When voting is active, each participant cannot see other participants."
                 let can_see = match app.role_input { // My role
                     Role::ScrumMaster => true, // SM sees all usually? Or maybe not. Let's assume SM sees.
                     _ => {
                         if let Phase::Voting { .. } = state.phase {
                             Some(player.id) == app.self_id
                         } else {
                             true
                         }
                     }
                 };

                 if can_see {
                    f.render_widget(Paragraph::new(symbol).style(style), area);
                    // Name label above?
                    if y > 0 {
                         let name_area = Rect { x: inner_rect.x + x, y: inner_rect.y + y - 1, width: player.name.len() as u16, height: 1 };
                         // f.render_widget(Paragraph::new(player.name.as_str()).style(Style::default().fg(Color::DarkGray)), name_area);
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
        
        let bottom = Paragraph::new(bottom_text)
            .block(Block::default().borders(Borders::ALL).title("Info"));
        f.render_widget(bottom, chunks[2]);
    }
}
