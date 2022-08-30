use gui::Button;

pub struct WindowsButton;

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows button has pressed");
    }
}
