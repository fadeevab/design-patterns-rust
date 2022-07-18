mod button;

use button::{id::IdButton, title::TitleButton, Button};

trait CreatableButton: Button {
    fn create() -> Self;
}

impl CreatableButton for TitleButton {
    fn create() -> TitleButton {
        TitleButton::new("Button".to_string())
    }
}

impl CreatableButton for IdButton {
    fn create() -> IdButton {
        IdButton::new(123)
    }
}

fn create_button<T: CreatableButton>() -> T {
    T::create()
}

fn render_dialog<T: CreatableButton>() {
    println!("--- Title ---");
    let button = create_button::<T>();
    button.render();
    println!("-------------");
}

fn main() {
    render_dialog::<TitleButton>();
    render_dialog::<IdButton>();
}
