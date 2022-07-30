use super::Component;

pub struct Folder {
    name: String,
    components: Vec<Box<dyn Component>>,
}

impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            components: vec![],
        }
    }

    pub fn add<C: Component + 'static>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }
}

impl Component for Folder {
    fn search(&self, keyword: &String) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        for component in self.components.iter() {
            component.search(keyword);
        }
    }
}
