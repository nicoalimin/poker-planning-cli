use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use rand::Rng;

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
    Orange,
    Pink,
    Purple,
    Mint,
    Gold,
    Silver,
    Bronze,
    Lime,
    Teal,
    Indigo,
    Violet,
    Coral,
    Crimson,
    White
}

impl AvatarColor {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let variants = [
            Self::Red, Self::Green, Self::Blue, Self::Yellow, Self::Magenta, Self::Cyan,
            Self::Orange, Self::Pink, Self::Purple, Self::Mint, Self::Gold, Self::Silver,
            Self::Bronze, Self::Lime, Self::Teal, Self::Indigo, Self::Violet, Self::Coral,
            Self::Crimson, Self::White
        ];
        variants[rng.gen_range(0..variants.len())].clone()
    }
    
    pub fn next(&self) -> Self {
         let variants = [
            Self::Red, Self::Green, Self::Blue, Self::Yellow, Self::Magenta, Self::Cyan,
            Self::Orange, Self::Pink, Self::Purple, Self::Mint, Self::Gold, Self::Silver,
            Self::Bronze, Self::Lime, Self::Teal, Self::Indigo, Self::Violet, Self::Coral,
            Self::Crimson, Self::White
        ];
        let pos = variants.iter().position(|r| r == self).unwrap_or(0);
        variants[(pos + 1) % variants.len()].clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AvatarSymbol {
    Human,
    Alien,
    Robot,
    Ghost,
    Cat,
    Dog,
    Bird,
    Fish,
    Tree,
    Flower,
    Star,
    Moon,
    Sun,
    Heart,
    Skull,
    Smile,
    Zap,
    Anchor,
    Music,
    Globe
}

impl AvatarSymbol {
    pub fn random() -> Self {
        let mut rng = rand::rng();
         let variants = [
            Self::Human, Self::Alien, Self::Robot, Self::Ghost, Self::Cat, Self::Dog,
            Self::Bird, Self::Fish, Self::Tree, Self::Flower, Self::Star, Self::Moon,
            Self::Sun, Self::Heart, Self::Skull, Self::Smile, Self::Zap, Self::Anchor,
            Self::Music, Self::Globe
        ];
        variants[rng.gen_range(0..variants.len())].clone()
    }
     pub fn next(&self) -> Self {
         let variants = [
            Self::Human, Self::Alien, Self::Robot, Self::Ghost, Self::Cat, Self::Dog,
            Self::Bird, Self::Fish, Self::Tree, Self::Flower, Self::Star, Self::Moon,
            Self::Sun, Self::Heart, Self::Skull, Self::Smile, Self::Zap, Self::Anchor,
            Self::Music, Self::Globe
        ];
        let pos = variants.iter().position(|r| r == self).unwrap_or(0);
        variants[(pos + 1) % variants.len()].clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub position: (u16, u16),
    pub color: AvatarColor,
    pub symbol: AvatarSymbol,
    #[serde(default)] // For backward compatibility if needed, though we don't have persistence
    pub confirmed: bool,
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
    VoteConfirm { confirmed: bool },
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
