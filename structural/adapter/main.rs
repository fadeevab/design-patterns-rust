mod adaptee;
mod adapter;

use adaptee::Adaptee;
use adapter::Adapter;

trait Target {
    fn request(&self) -> String;
}

fn client(target: impl Target) {
    println!("'{}'", target.request());
}

fn main() {
    let adaptee = Adaptee;

    println!(
        "Adaptee is incompatible with the client: '{}'",
        adaptee.specific_request()
    );

    let adapter = Adapter::new(adaptee);

    print!("But with adapter client can call it's method: ");
    client(adapter);
}
