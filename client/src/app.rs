use common::{GameState, Phase, Role, VotingConfig};
use uuid::Uuid;
use tui_input::Input;

pub enum CurrentScreen {
    Login,
    Main,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub server_url: String, // Input for server URL
    pub name_input: Input,
    pub role_input: Role, 
    pub color_input: common::AvatarColor,
    pub symbol_input: common::AvatarSymbol,
    
    pub game_state: Option<GameState>,
    pub self_id: Option<Uuid>,

    // For movement input
    pub last_known_pos: (u16, u16),
    
    // Logs
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Login,
            server_url: "127.0.0.1:8888".to_string(),
            name_input: Input::default(),
            role_input: Role::Participant,
            color_input: common::AvatarColor::Yellow,
            symbol_input: common::AvatarSymbol::Human,
            game_state: None,
            self_id: None,
            last_known_pos: (10, 10),
            logs: Vec::new(),
        }
    }
    
    pub fn log(&mut self, msg: String) {
        // Keep last 10 logs
        if self.logs.len() >= 10 {
            self.logs.remove(0);
        }
        self.logs.push(format!("{} - {}", common::current_time_unix(), msg));
    }
}
