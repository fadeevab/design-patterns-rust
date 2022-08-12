#[derive(Clone)]
struct Circle {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
}

fn main() {
    let circle1 = Circle {
        x: 10,
        y: 15,
        radius: 10,
    };

    // Prototype in action.
    let mut circle2 = circle1.clone();
    circle2.radius = 77;

    println!("Circle 1: {}, {}, {}", circle1.x, circle1.y, circle1.radius);
    println!("Circle 2: {}, {}, {}", circle2.x, circle2.y, circle2.radius);
}
