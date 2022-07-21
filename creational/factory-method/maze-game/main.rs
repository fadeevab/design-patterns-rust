mod game;

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
