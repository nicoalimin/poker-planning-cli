use common::{VotingConfig};

pub struct Zone {
    pub value: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub fn calculate_zones(config: &VotingConfig) -> Vec<Zone> {
    // Layout in Grid Units (1 unit = 2x1 chars from ui.rs scale)
    let mut zones = Vec::new();
    let start_x = 2; // Offset slightly
    let start_y = 4; // Move down a bit to leave room for players at top
    let spacing = 5; // 4 width + 1 gap
    
    for (i, &card) in config.cards.iter().enumerate() {
        zones.push(Zone {
            value: card,
            x: start_x + (i as u16 * spacing),
            y: start_y,
            width: 4, // 4 grid units wide (8 chars)
            height: 4, // 4 grid units high (4 chars)
        });
    }
    zones
}
