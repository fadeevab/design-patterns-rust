//! Dynamic dispatch affects performance in runtime, thus, you might
//! prefer implementation via generics to make the compiler figure out a proper
//! factory type.

mod gui;

/// The client code calls the creation methods of a factory object instead of
/// creating products directly with a constructor call.
fn render(factory: &dyn gui::GuiFactoryDynamic) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}

fn main() {
    use gui::{macos::MacFactory, windows::WindowsFactory};

    let windows = false;

    // Allocate a factory object in runtime depending on unpredictable input.
    let factory: &dyn gui::GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    // Factory invocation can be inlined right here then.
    let button = factory.create_button();
    button.press();

    // Factory object can be passed to a function as a parameter.
    render(factory);
}
