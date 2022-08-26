use gui::button::Button;
use gui::dialog::Dialog;

use crate::button::WindowsButton;

pub struct WindowsDialog;

impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}
