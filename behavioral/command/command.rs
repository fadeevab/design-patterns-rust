mod copy;
mod cut;
mod paste;

pub use copy::CopyCommand;
pub use cut::CutCommand;
pub use paste::PasteCommand;

/// Declares a method for executing (and undoing) a command.
///
/// Each command receives an application context to access
/// visual components (e.g. edit view) and a clipboard.
pub trait Command {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool;
    fn undo(&mut self, app: &mut cursive::Cursive);
}
