//@path = "src/universe.rs"

use crate::gameconfig;
use crate::sector::Sector;
use pathfinding::prelude::dijkstra;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Universe {
    sectors: HashMap<u32, Sector>,
    created_at: SystemTime,
}

impl Universe {
    pub fn new(sectors: HashMap<u32, Sector>) -> Universe {
        Universe {
            sectors: sectors.clone(),
            created_at: SystemTime::now(),
        }
    }
    pub fn shortest_path(&self, start: u32, end: u32) -> Option<Vec<u32>> {
        let result = dijkstra(
            &start,
            |&node| {
                self.sectors[&node]
                    .connections()
                    .iter()
                    .map(|&i| (i, 1))
                    .collect::<Vec<(u32, u32)>>()
            },
            |&node| node == end,
        );
    

        result.map(|(path, _cost)| path)
    }
}

//getter implementation for Universe
impl Universe {
    pub fn _sectors(&self) -> &HashMap<u32, Sector> {
        &self.sectors
    }

    pub fn sector(&self, id: u32) -> Option<&Sector> {
        self.sectors.get(&id)
    }

    pub fn created_at(&self) -> &SystemTime {
        &self.created_at
    }
}

pub fn big_bang_start() -> HashMap<u32, Sector> {
    // Load the game configuration
    let config = gameconfig::Gamefig::from_file("gamefig.toml").unwrap();

    // Generate the universe from the configuration
    big_bang(config.game_config.universe_size_in_sectors, config.game_config.random_seed)
}

/* The code above does the following:
    1. Initialize all sectors with empty connections
    2. Iterates over all sectors and for each sector:
        - Generates a random number of connections to make to other sectors
        - Generates a random sector to connect to
        - Adds a connection from the current sector to the randomly selected sector
        - Adds a connection from the randomly selected sector to the current sector
    3. Removes all connections from the first three sectors to sectors with an id greater than 10
        - While the number of connections is less than 2, add a new connection with a random ID between 1 and 10.
*/

fn big_bang(num_sectors: u32, random_seed: u64) -> HashMap<u32, Sector> {
    let mut sectors: HashMap<u32, Sector> = HashMap::new();

    // Use the provided seed to create a seeded RNG
    let mut rng = rand::rngs::StdRng::seed_from_u64(random_seed);

    for sector_id in 1..=num_sectors {
        sectors.insert(
            sector_id,
            Sector::new(sector_id, format!("Sector {}", sector_id), Vec::new()),
        ); // initialize all sectors with empty connections
    }

    for sector_id in 1..=num_sectors {
        let num_connections = if sector_id <= 10 {
            rng.gen_range(1..=4)
        } else {
            rng.gen_range(1..=6)
        };
        for _ in 0..num_connections {
            let connected_sector_id = if sector_id <= 10 && rng.gen_bool(0.8) {
                rng.gen_range(1..=10)
            } else {
                rng.gen_range(1..=num_sectors)
            };
            if connected_sector_id != sector_id
                && sectors
                    .get(&connected_sector_id)
                    .unwrap()
                    .connections()
                    .len()
                    < 6
            {
                if let Some(sector) = sectors.get_mut(&sector_id) {
                    if !sector.connections().contains(&connected_sector_id) {
                        sector.add_connection(connected_sector_id);
                    }
                }
                if let Some(connected_sector) = sectors.get_mut(&connected_sector_id) {
                    if !connected_sector.connections().contains(&sector_id) {
                        connected_sector.add_connection(sector_id); // add a connection back to the original sector
                    }
                }
            }
        }
        if sectors.get(&sector_id).unwrap().connections().is_empty() {
            let connected_sector_id = if sector_id <= 10 {
                rng.gen_range(1..=10)
            } else {
                rng.gen_range(1..=num_sectors)
            };
            if sectors
                .get(&connected_sector_id)
                .unwrap()
                .connections()
                .len()
                < 6
            {
                if let Some(sector) = sectors.get_mut(&sector_id) {
                    sector.add_connection(connected_sector_id);
                }
                if let Some(connected_sector) = sectors.get_mut(&connected_sector_id) {
                    connected_sector.add_connection(sector_id); // add a connection back to the original sector
                }
            }
        }
    }

    for sector_id in 1..=3 {
        if let Some(sector) = sectors.get_mut(&sector_id) {
            sector.remove_connections(|&connection| connection > 10);
            while sector.connections().len() < 2 {
                let new_connection = rng.gen_range(1..=10);
                if new_connection != sector_id && !sector.connections().contains(&new_connection) {
                    sector.add_connection(new_connection);
                } else if sector.connections().len() == 10 {
                    // All possible connections are already made, break the loop to avoid infinite loop
                    break;
                }
            }
        }
    }

    sectors
}
