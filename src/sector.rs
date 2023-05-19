//@path = "src/sector.rs"

use std::io::Write;

use crate::player::Player;

#[derive(Clone)]
pub struct Sector {
    id: u32,
    name: String,
    connections: Vec<u32>,
}

impl Sector {
    pub fn new(id: u32, name: String, connections: Vec<u32>) -> Sector {
        Sector {
            id,
            name,
            connections,
        }
    }
    //add_connection method
    pub fn add_connection(&mut self, connection: u32) {
        if !self.connections.contains(&connection) {
            // avoid duplicates
            self.connections.push(connection);
        }
    }
    // Remove a connection from the sector
    pub fn _remove_connection(&mut self, connection: u32) {
        if let Some(index) = self.connections.iter().position(|&c| c == connection) {
            // find the index of the connection
            self.connections.remove(index); // remove it from the vector
        }
    }
    // Remove connections that satisfy a predicate
    pub fn remove_connections<P>(&mut self, mut predicate: P)
    where
        P: FnMut(&u32) -> bool,
    {
        self.connections.retain(|c| !predicate(c)); // retain only the connections that do not satisfy the predicate
    }
}

// Getter methods
impl Sector {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn connections(&self) -> &Vec<u32> {
        &self.connections
    }
}

pub fn display_sector(sector: &Sector, player: &Player, automove: bool) {
    // Display sector name with '(unexplored)' tag if the player hasn't visited the sector yet
    let visited = if player.has_visited(sector.id()) {
        ""
    } else {
        " (unexplored)"
    };
    println!("Sector  : {} in {}{}", sector.id(), sector.name(), visited);

    // Display connections with sector number within parentheses for unexplored sectors
    let connections: Vec<String> = sector
        .connections()
        .iter()
        .map(|id| {
            if player.has_visited(*id) {
                id.to_string()
            } else {
                format!("({})", id)
            }
        })
        .collect();
    println!("Adjacent Sector(s) :  {}", connections.join(" - "));
    if !automove {
        print!("\nCommand : [{}] [{}] ? :", player.name(), sector.id());
        std::io::stdout().flush().unwrap();
    }
}
