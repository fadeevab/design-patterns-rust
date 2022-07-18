pub mod html;
pub mod windows;

pub trait Button {
    fn render(&self);
}

pub trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;

    fn render(&self) {
        let button = self.create_button();
        button.render();
    }

    fn refresh(&self) {
        println!("Refresh");
    }
}
