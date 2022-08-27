mod game;
mod magic_maze;
mod ordinary_maze;

use magic_maze::MagicMaze;
use ordinary_maze::OrdinaryMaze;

fn main() {
    let magic_maze = MagicMaze::new();
    game::run(magic_maze);

    let ordinary_maze = OrdinaryMaze::new();
    game::run(ordinary_maze);
}
