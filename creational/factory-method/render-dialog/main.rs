mod button;
mod dialog;

use dialog::{html::HtmlDialog, windows::WindowsDialog, Dialog};

fn main() {
    // The dialog factory is constructed depending on unpredictable input
    // (not really unpredictable in this case, but let's imagine it).
    // We need to put a new object into a `Box` pointer, then it's going to be
    // used uniformly throughout a code.
    let dialog: Box<dyn Dialog> = if cfg!(windows) {
        Box::new(WindowsDialog)
    } else {
        Box::new(HtmlDialog)
    };

    dialog.render();
    dialog.refresh();
}
