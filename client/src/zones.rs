use common::{VotingConfig};

pub struct Zone {
    pub value: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub fn calculate_zones(config: &VotingConfig) -> Vec<Zone> {
    // Layout in Grid Units (1 unit = 4x2 chars)
    let mut zones = Vec::new();
    let start_x = 1;
    let start_y = 2;
    let spacing = 3; // 3 grid units apart
    
    for (i, &card) in config.cards.iter().enumerate() {
        zones.push(Zone {
            value: card,
            x: start_x + (i as u16 * spacing),
            y: start_y,
            width: 2, // 2 grid units wide
            height: 2, // 2 grid units high
        });
    }
    zones
}
