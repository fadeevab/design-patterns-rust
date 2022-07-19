struct User {
    name: String,
    surname: String,
}

impl User {
    // "Static creation method" is actually a Rust's idiomatic way to define a "constructor".
    pub fn new(name: String, surname: String) -> Self {
        Self { name, surname }
    }

    // Constructs an object by reading from database.
    pub fn load(id: u32) -> Self {
        if id == 42 {
            Self {
                name: "John".into(),
                surname: "Smith".into(),
            }
        } else {
            panic!()
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn surname(&self) -> &String {
        &self.surname
    }
}

fn main() {
    let alice = User::new("Alice".into(), "Fisher".into());
    let john = User::load(42);

    println!("{} {}", alice.name(), alice.surname());
    println!("{} {}", john.name(), john.surname());
}
