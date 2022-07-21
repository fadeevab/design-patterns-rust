//! Taken from: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//!
//! Rust doesn't really allows a singleton pattern without `unsafe` because it
//! doesn't have a safe mutable global state!
//!
//! Crates like `lazy-static` are actually implemented via `unsafe` with
//! `static mut` manipulation.
//!
//! Better: use no global variables at all and pass everything around.
//! The oldest living variable is an object created at the start of the `main()`.

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {}", ARRAY.lock().unwrap().len());
}
