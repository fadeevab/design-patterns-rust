mod game;
mod magic_maze;
mod ordinary_maze;

use magic_maze::MagicMaze;
use ordinary_maze::OrdinaryMaze;

/// The game runs with different mazes depending on the concrete factory type:
/// it's either an ordinary maze or a magic maze.
///
/// For demonstration purposes, both mazes are used to construct the game.
fn main() {
    // Option 1: The game starts with an ordinary maze.
    let ordinary_maze = OrdinaryMaze::new();
    game::run(ordinary_maze);

    // Option 2: The game starts with a magic maze.
    let magic_maze = MagicMaze::new();
    game::run(magic_maze);
}
