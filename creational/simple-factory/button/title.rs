use super::Button;

pub struct TitleButton {
    title: String,
}

impl TitleButton {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Button for TitleButton {
    fn render(&self) {
        println!("'{}'", self.title);
    }
}
