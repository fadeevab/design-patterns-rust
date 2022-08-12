mod game;

/// The client code demonstrates that it does its own things
/// (e.g. loading resources) and it can use a factory in a proper place to
/// construct a game.
fn run(game: impl game::MazeGame) {
    println!("Loading resources...");
    println!("Starting the game...");

    game.play();
}

fn main() {
    let game = game::MagicMazeGame::new();
    run(game);

    let game = game::OrdinaryMazeGame::new();
    run(game);
}
