use crate::button::Button;

/// Dialog has a factory method `create_button`.
///
/// It creates different buttons depending on a factory implementation.
pub trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;

    fn render(&self) {
        println!("Dialog: Using base render method.");
        let button = self.create_button();
        button.render();
    }

    fn refresh(&self) {
        println!("Dialog: Using base refresh method.");
    }
}