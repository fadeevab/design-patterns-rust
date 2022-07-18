use super::{Button, Checkbox, GuiFactoryDynamic, GuiFactory};

pub struct MacButton;
pub struct MacCheckbox;
pub struct MacFactory;

impl Button for MacButton {
    fn press(&self) {
        println!("MacOS button");
    }
}

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("MacOS checkbox");
    }
}

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckbox;

    fn create_button(&self) -> MacButton {
        MacButton
    }

    fn create_checkbox(&self) -> MacCheckbox {
        MacCheckbox
    }
}

impl GuiFactoryDynamic for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
