use crate::gui::{Button, Dialog};

pub struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self) {
        println!("<button>Test Button</button>");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Button says - 'Hello World!'");
    }
}

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
    /// Creates an HTML button.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}
