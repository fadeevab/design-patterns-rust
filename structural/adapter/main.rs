mod adaptee;
mod adapter;
mod target;

use adaptee::SpecificTarget;
use adapter::TargetAdapter;
use target::{OrdinaryTarget, Target};

fn call_client(target: impl Target) {
    println!("'{}'", target.request());
}

fn main() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call_client(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{}'",
        adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call_client(adapter);
}
