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
    pub role_input: Role, // Cycle through roles?
    
    pub game_state: Option<GameState>,
    pub self_id: Option<Uuid>,

    // For movement input
    pub last_known_pos: (u16, u16),
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Login,
            server_url: "127.0.0.1:8888".to_string(),
            name_input: Input::default(),
            role_input: Role::Participant,
            game_state: None,
            self_id: None,
            last_known_pos: (10, 10),
        }
    }
}
