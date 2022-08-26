use gui::button::Button;

pub struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self) {
        println!("<button>Test Button</button>");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Button: I'm an HTML button!");
    }
}
