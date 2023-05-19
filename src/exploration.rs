//@path = "src/exploration.rs"
use crate::player::Player;

use crate::sector::{self};
use crate::universe::Universe;
use std::io::Write;

// Helper function to handle moving to a target sector
pub fn handle_move(universe: &Universe, player: &mut Player, target_sector_id: u32) {
    let current_sector = universe.sector(player.current_sector()).unwrap();
    if current_sector.connections().contains(&target_sector_id) {
        // If the target sector is directly connected, move immediately
        player.visit(target_sector_id);
    } else {
        handle_automovement(universe, player, target_sector_id);
    }
}

fn handle_automovement(universe: &Universe, player: &mut Player, target_sector_id: u32) {
    if let Some(path) = universe.shortest_path(player.current_sector(), target_sector_id) {
        // If there is a shortest path, ask for auto movement
        println!();
        println!();
        println!("That sector is not adjacent.");
        let path_display: Vec<String> = path
            .iter()
            .map(|&i| {
                if player.has_visited(i) {
                    i.to_string()
                } else {
                    format!("({})", i)
                }
            })
            .collect();
        println!(
            "The shortest path ({} hops) from sector {} to sector {} is: {}",
            path.len(),
            player.current_sector(),
            target_sector_id,
            path_display.join(" > ")
        );
        println!();
        println!("Engage auto movement? [Y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("Y") {
            // If auto movement is confirmed, move along the path
            move_along_path(universe, player, path);
        }
    } else {
        // If there is no path, print an error message
        println!(
            "There's no path from sector {} to sector {}.",
            player.current_sector(),
            target_sector_id
        );
    }
}

// Helper function to move along a path of sectors
fn move_along_path(universe: &Universe, player: &mut Player, path: Vec<u32>) {
    // Use enumerate to get the index of each sector
    for (index, sector_id) in path.iter().enumerate() {
        player.visit(*sector_id);
        // Display a loading bar
        for _ in 0..5 {
            print!("=");
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(300));
        }
        print!(">");
        println!();
        println!("Moving to Sector {} ...", sector_id);
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!();
        // Check if it is the last sector using len
        if index < path.len() - 1 {
            let current_sector = universe.sector(player.current_sector()).unwrap();
            sector::display_sector(current_sector, player, true);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
