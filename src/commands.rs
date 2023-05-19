//@path = "src/commands.rs"

use crate::{
    exploration::handle_move,
    player::Player,
    sector,
    universe::Universe,
};

pub fn run_game_loop(universe: &Universe, player: &mut Player) {
    let mut input = String::new();

    loop {
        let current_sector = universe.sector(player.current_sector()).unwrap();
        sector::display_sector(current_sector, player, false);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "/" => player.display_basic_info(),
            _ => sector_menu_command(universe, player, command),
        }
    }
}

fn sector_menu_command(universe: &Universe, player: &mut Player, command: &str) {
    match command {
        "" => {
            println!("<Re-Display>");
            println!();
        }
        "quit" | "q" => {
            if confirm_quit() {
                std::process::exit(0);
            }
        }
        command => {
            if let Ok(target_sector_id) = command.parse::<u32>() {
                handle_move(universe, player, target_sector_id);
            } else {
                println!("Unknown command.");
            }
        }
    }
}

// Helper function to confirm quitting the game
fn confirm_quit() -> bool {
    let mut input = String::new();
    println!("Are you sure you want to quit? [Y/N]");
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("Y")
}
