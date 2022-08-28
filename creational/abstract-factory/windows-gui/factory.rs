use gui::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

use crate::{button::WindowsButton, checkbox::WindowsCheckbox};

pub struct WindowsFactory;

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
