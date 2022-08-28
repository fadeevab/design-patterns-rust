use gui::Checkbox;

pub struct MacCheckbox;

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("MacOS checkbox has switched");
    }
}
