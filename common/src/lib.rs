use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    ScrumMaster,
    Participant,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AvatarColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AvatarSymbol {
    Human,
    Alien,
    Robot,
    Ghost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub position: (u16, u16),
    pub color: AvatarColor,
    pub symbol: AvatarSymbol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Phase {
    Idle,
    Voting {
        start_time_unix: u64,
        duration_secs: Option<u64>,
    },
    Revealed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: HashMap<Uuid, Player>,
    pub phase: Phase,
    pub current_ticket: Option<Ticket>,
    // PlayerId -> Vote value. 
    // On the wire, hidden votes should be masked unless Phase is Revealed.
    pub votes: HashMap<Uuid, Option<u32>>,
    pub config: VotingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingConfig {
    pub cards: Vec<u32>,
    pub default_timeout: Option<u64>,
}

impl Default for VotingConfig {
    fn default() -> Self {
        Self {
            cards: vec![0, 1, 2, 3, 5, 8, 13],
            default_timeout: Some(20),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientPayload {
    Login { name: String, role: Role, color: AvatarColor, symbol: AvatarSymbol },
    Move { x: u16, y: u16 },
    Vote { value: Option<u32> },
    Admin(AdminCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdminCommand {
    StartVote {
        ticket: Option<Ticket>,
        timeout: Option<u64>,
    },
    Reveal,
    Reset,
    Kick { player_id: Uuid },
    UpdateConfig(VotingConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerPayload {
    Welcome { self_id: Uuid, state: GameState },
    StateUpdate(GameState),
    Error(String),
}

pub fn current_time_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
