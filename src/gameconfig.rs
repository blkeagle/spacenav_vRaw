//@path = "src/gameconfig.rs"

use std::fs;
use serde::Deserialize;
use toml::de::Error;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Gamefig {
    pub game_config: GameConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GameConfig {
    pub random_seed: u64,
    pub universe_size_in_sectors: u32,
    maximum_possible_players: u32,
    standard_starport_power: u32,
    special_starport_power: u32,
    maximum_possible_starports: u32,
    initial_starports_to_build: u32,
    maximum_possible_planets: u32,
    two_way_warps: u32,
    one_way_warps: u32,
    internal_alien_traders: bool,
}

impl Gamefig {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");

        toml::from_str(&contents)
    }
}