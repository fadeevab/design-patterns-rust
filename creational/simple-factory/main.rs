mod button;

use button::{id::IdButton, title::TitleButton, Button};

/// Creates a button depending on a parameter value, it is the simple factory.
fn create_button(random_number: f64) -> Box<dyn Button> {
    if random_number < 0.5 {
        Box::new(TitleButton::new("Button".to_string()))
    } else {
        Box::new(IdButton::new(123))
    }
}

fn render_dialog(random_number: f64) {
    println!("--- Title ---");
    let button = create_button(random_number);
    button.render();
    println!("-------------");
}

fn main() {
    render_dialog(0.3);
    render_dialog(0.6);
}
