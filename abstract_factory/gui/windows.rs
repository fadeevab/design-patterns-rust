use super::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

pub struct WindowsButton;
pub struct WindowsCheckbox;
pub struct WindowsFactory;

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows button");
    }
}

impl Checkbox for WindowsCheckbox {
    fn switch(&self) {
        println!("Windows checkbox");
    }
}

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> WindowsButton {
        WindowsButton
    }

    fn create_checkbox(&self) -> WindowsCheckbox {
        WindowsCheckbox
    }
}

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}
