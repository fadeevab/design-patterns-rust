mod magic_maze;
mod ordinary_maze;

pub use magic_maze::*;
pub use ordinary_maze::*;

pub trait Room {
    fn render(&self);
}

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
