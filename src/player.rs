//@path = "src/player.rs"

pub struct Player {
    name: String,
    current_sector: u32,
    visited_sectors: Vec<u32>,
}

impl Player {
    pub fn new(name: String) -> Player {
        let mut player = Player {
            name,
            current_sector: 1,
            visited_sectors: Vec::new(),
        };
        player.visit(1); // Mark sector 1 as visited at the start of the game
        player
    }

    pub fn visit(&mut self, sector_id: u32) {
        if !self.visited_sectors.contains(&sector_id) {
            self.visited_sectors.push(sector_id);
        }
        self.current_sector = sector_id; // Update the current sector
    }

    pub fn has_visited(&self, sector_id: u32) -> bool {
        self.visited_sectors.contains(&sector_id)
    }
}

// Getter methods for Player
impl Player {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn _visited_sectors(&self) -> &Vec<u32> {
         &self.visited_sectors
    }
    pub fn current_sector(&self) -> u32 {
        self.current_sector
    }
}
