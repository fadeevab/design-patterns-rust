//! Generics allow the compiler to create a code that doesn't require
//! dynamic dispatch in runtime aka virtual method invocation.

mod gui;

// Factory object should be passed as a parameter to a generic function
// with a client code that contains factory invocation.
fn render(factory: impl gui::GuiFactory) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    use gui::{Button, Checkbox};

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}

fn main() {
    use gui::{macos::MacFactory, windows::WindowsFactory};

    let windows = true;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}
