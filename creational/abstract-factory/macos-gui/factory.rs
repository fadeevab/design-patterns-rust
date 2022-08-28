use gui::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

use crate::{button::MacButton, checkbox::MacCheckbox};

pub struct MacFactory;

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
