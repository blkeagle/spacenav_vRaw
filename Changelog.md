# Change Log

## Main Features

1. The game starts by asking the user for their name and the number of sectors they want in the universe.
2. The universe is then created with the specified number of sectors, each with a random number of connections to other sectors.
3. The player starts in sector 1, which is marked as visited.
4. The game enters a loop where it displays the current sector and waits for a command from the user.
5. The user can enter a sector number to move to that sector. If the sector is directly connected to the current sector, the player moves immediately. If not, the game calculates the shortest path to that sector and asks the user if they want to engage auto movement.
6. If the user confirms auto movement, the game moves the player along the shortest path to the target sector, displaying a loading bar and a message for each sector along the way.
7. The user can also enter "quit" or "q" to quit the game. The game will ask for confirmation before quitting.

## Code Structure

1. The code is organized into several modules: `main`, `commands`, `exploration`, `player`, `sector`, and `universe`.
2. The `main` module handles the game setup and main loop.
3. The `commands` module handles user commands and the game loop.
4. The `exploration` module handles moving between sectors.
5. The `player` module defines the `Player` struct and its methods.
6. The `sector` module defines the `Sector` struct and its methods, as well as a function to display a sector.
7. The `universe` module defines the `Universe` struct and its methods, as well as a function to create a new universe.

## DataStructures

Game
├── Player
│   ├── name: String
│   ├── current_sector: u32
│   └── visited_sectors: Vec< u32>
├── Universe
│   ├── sectors: HashMap< u32, Sector>
│   └── created_at: SystemTime
├── Sector
│   ├── id: u32
│   ├── name: String
│   └── connections: Vec< u32>
└── GameConfig
    ├── random_seed: String
    ├── universe_size_in_sectors: u32
    ├── maximum_possible_players: u32
    ├── standard_starport_power: u32
    ├── special_starport_power: u32
    ├── maximum_possible_starports: u32
    ├── initial_starports_to_build: u32
    ├── maximum_possible_planets: u32
    ├── two_way_warps: u32
    ├── one_way_warps: u32
    └── internal_alien_traders: bool

## FileStructure

spacenav/
├── Cargo.toml
├── gameconfig.toml
└── src/
    ├── main.rs
    ├── commands.rs
    ├── exploration.rs
    ├── player.rs
    ├── sector.rs
    ├── universe.rs
    └── gameconfig.rs

## Recommendations and Next Steps

1. Add more commands for the user to interact with the game. For example, you could add commands to display a map of the universe, to display the player's visited sectors, or to save and load the game state.
2. Improve the display of sectors. For example, you could color-code the sectors based on whether they have been visited or not, or you could display the sectors in a graphical format instead of just text.
3. Add more features to the universe. For example, you could add special events that occur in certain sectors, or you could add items that the player can find and collect.
4. Add more complexity to the movement between sectors. For example, you could add a fuel system that limits how far the player can move without refueling, or you could add hazards that the player must avoid when moving between sectors.
5. Consider adding tests to ensure the functionality of your code as it grows in complexity.
6. Consider adding error handling for user input to ensure the game doesn't crash if the user enters an invalid command or sector number.
