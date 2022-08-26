use gui::dialog::Dialog;
use html_gui::dialog::HtmlDialog;
use windows_gui::dialog::WindowsDialog;

pub fn initialize() -> Box<dyn Dialog> {
    // The dialog type is selected depending on the environment settings or configuration.
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        return Box::new(WindowsDialog)
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        return Box::new(HtmlDialog)
    };
}