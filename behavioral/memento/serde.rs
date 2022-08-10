use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Originator {
    state: u32,
}

impl Originator {
    pub fn save(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn restore(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

fn main() {
    let mut history = Vec::<String>::new();

    let mut originator = Originator { state: 0 };

    originator.state = 1;
    history.push(originator.save());

    originator.state = 2;
    history.push(originator.save());

    for moment in history.iter() {
        println!("{}", moment);
    }

    let originator = Originator::restore(&history.pop().unwrap());
    println!("Restored to state: {}", originator.state);

    let originator = Originator::restore(&history.pop().unwrap());
    println!("Restored to state: {}", originator.state);
}
