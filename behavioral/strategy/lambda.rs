type RouteStrategy = fn(from: &str, to: &str);

fn walking_strategy(from: &str, to: &str) {
    println!("Walking route from {} to {}: 4 km, 30 min", from, to);
}

fn public_transport_strategy(from: &str, to: &str) {
    println!(
        "Public transport route from {} to {}: 3 km, 5 min",
        from, to
    );
}

struct Navigator {
    route_strategy: RouteStrategy,
}

impl Navigator {
    pub fn new(route_strategy: RouteStrategy) -> Self {
        Self { route_strategy }
    }

    pub fn route(&self, from: &str, to: &str) {
        (self.route_strategy)(from, to);
    }
}

fn main() {
    let navigator = Navigator::new(walking_strategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(public_transport_strategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(|from, to| println!("Specific route from {} to {}", from, to));
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}
