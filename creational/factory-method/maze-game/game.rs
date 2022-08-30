/// Maze room that is going to be instantiated with a factory method.
pub trait Room {
    fn render(&self);
}

/// Maze game has a factory method producing different rooms.
pub trait MazeGame {
    type RoomImpl: Room;

    /// A factory method.
    fn rooms(&self) -> Vec<Self::RoomImpl>;

    fn play(&self) {
        for room in self.rooms() {
            room.render();
        }
    }
}

/// The client code initializes resources and does other preparations
/// then it uses a factory to construct and run the game.
pub fn run(maze_game: impl MazeGame) {
    println!("Loading resources...");
    println!("Starting the game...");

    maze_game.play();
}
