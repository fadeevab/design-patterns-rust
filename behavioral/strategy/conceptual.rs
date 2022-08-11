/// Defines an injectable strategy for building routes.
trait RouteStrategy {
    fn build_route(&self, from: &str, to: &str);
}

struct WalkingStrategy;

impl RouteStrategy for WalkingStrategy {
    fn build_route(&self, from: &str, to: &str) {
        println!("Walking route from {} to {}: 4 km, 30 min", from, to);
    }
}

struct PublicTransportStrategy;

impl RouteStrategy for PublicTransportStrategy {
    fn build_route(&self, from: &str, to: &str) {
        println!(
            "Public transport route from {} to {}: 3 km, 5 min",
            from, to
        );
    }
}

struct Navigator<T: RouteStrategy> {
    route_strategy: T,
}

impl<T: RouteStrategy> Navigator<T> {
    pub fn new(route_strategy: T) -> Self {
        Self { route_strategy }
    }

    pub fn route(&self, from: &str, to: &str) {
        self.route_strategy.build_route(from, to);
    }
}

fn main() {
    let navigator = Navigator::new(WalkingStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");

    let navigator = Navigator::new(PublicTransportStrategy);
    navigator.route("Home", "Club");
    navigator.route("Club", "Work");
}
