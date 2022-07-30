use super::Component;

pub struct File {
    name: String,
}

impl File {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Component for File {
    fn search(&self, keyword: &String) {
        println!("Searching for keyword {} in file {}", keyword, self.name);
    }
}
