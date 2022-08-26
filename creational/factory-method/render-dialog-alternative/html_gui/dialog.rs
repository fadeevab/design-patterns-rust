use gui::button::Button;
use gui::dialog::Dialog;

use crate::button::HtmlButton;

pub struct HtmlDialog;

impl Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton)
    }
}
