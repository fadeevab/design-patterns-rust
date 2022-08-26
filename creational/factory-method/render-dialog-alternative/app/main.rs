use gui::dialog::Dialog;

mod init;
use crate::init::initialize;

fn main() {
    // The rest of the code doesn't depend on specific dialog types, because it works with all
    // dialog objects via the abstract Dialog trait.
    let dialog: Box<dyn Dialog> = initialize();
    dialog.render();
    dialog.refresh();
}
