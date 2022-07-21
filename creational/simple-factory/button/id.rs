use super::Button;

pub struct IdButton {
    id: u32,
}

impl IdButton {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

impl Button for IdButton {
    fn render(&self) {
        println!("Button #{}", self.id);
    }
}
