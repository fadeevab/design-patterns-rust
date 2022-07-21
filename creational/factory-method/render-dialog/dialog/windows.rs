use super::Dialog;

use crate::button::{windows::WindowsButton, Button};

pub struct WindowsDialog;

impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }
}
