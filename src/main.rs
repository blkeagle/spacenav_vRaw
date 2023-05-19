//@path = "src/main.rs"

use crate::{
    player::Player,
    universe::{Universe},
};

mod gameconfig;
mod commands;
mod exploration;
mod player;
mod sector;
mod universe;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut input = String::new();
    
    clear_screen();
    println!("Enter your name:");
    std::io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    let mut player = Player::new(name);

    // Generate the universe
    let sectors = universe::big_bang_start();
    let universe = Universe::new(sectors.clone());
    
    clear_screen();
    println!(
        "Awesome! Welcome: {}! You are in sector 1. This Universe was created at {:?}.",
        player.name(),
        universe.created_at()
    );
    commands::run_game_loop(&universe, &mut player);
}