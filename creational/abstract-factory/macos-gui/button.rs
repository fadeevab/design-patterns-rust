use gui::Button;

pub struct MacButton;

impl Button for MacButton {
    fn press(&self) {
        println!("MacOS button has pressed");
    }
}
