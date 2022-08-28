mod gui;
mod html_gui;
mod init;
mod windows_gui;

use init::initialize;

fn main() {
    // The rest of the code doesn't depend on specific dialog types, because
    // it works with all dialog objects via the abstract `Dialog` trait
    // which is defined in the `gui` module.
    let dialog = initialize();
    dialog.render();
    dialog.refresh();
}
