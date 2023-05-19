//@path = "src/player.rs"

pub struct Player {
    name: String,
    pub current_sector: u32,
    pub last_visited_sector: u32,
    visited_sectors: Vec<u32>,
    last_online: String, // new field, consider using a date/time type
    rank: String, // new field
    experience: u32, // new field
    pub last_move_was_automovement: bool,
}

impl Player {
    pub fn new(name: String) -> Player {
        let mut player = Player {
            name,
            current_sector: 1,
            last_visited_sector: 1,
            visited_sectors: Vec::new(),
            last_online: "todo!()".to_string(),
            rank: "todo!()".to_string(),
            experience: 0,
            last_move_was_automovement: false,
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
    pub fn display_basic_info(&self) {
        println!("\n[Basic Information]\n");
        println!("Player Name: {}", self.name);
        println!("Last Day Online: {}", self.last_online);
        println!("Rank: {}", self.rank);
        println!("Experience: {}", self.experience);

        println!("\n[Navigation Information]\n");
        println!("Current Sector: {}", self.current_sector);
        if self.last_move_was_automovement {
            println!("Last Sector: {} (Automovement)", self.last_visited_sector);
        } else {
            println!("Last Sector: {}", self.last_visited_sector);
        }
        // println!("Universe Discovery: {} out of {} sectors ({}%)", 
        //     self.visited_sectors.len(), todo!(), todo!()); 
        //     /* total number of sectors */ 
        //     /* percentage of discovered sectors */
        println!("\n\n");
    }
}

// Getter methods for Player
#[allow(dead_code)]
impl Player {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn visited_sectors(&self) -> &Vec<u32> {
         &self.visited_sectors
    }
    pub fn current_sector(&self) -> u32 {
        self.current_sector
    }
}
