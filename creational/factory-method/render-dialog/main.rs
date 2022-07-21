mod button;
mod dialog;

use dialog::{html::HtmlDialog, windows::WindowsDialog, Dialog};

fn main() {
    let windows = false;

    let dialog: Box<dyn Dialog> = if windows {
        Box::new(WindowsDialog)
    } else {
        Box::new(HtmlDialog)
    };

    dialog.render();
    dialog.refresh();
}
