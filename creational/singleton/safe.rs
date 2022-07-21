//! Rust doesn't really allows a singleton pattern without `unsafe` because it
//! doesn't have a safe mutable global state!
//!
//! Crates like `lazy-static` are actually implemented via `unsafe` with
//! `static mut` manipulation.
//!
//! Better: use no global variables at all and pass everything around.
//! The oldest living variable is an object created at the start of the `main()`.

fn change(global_state: &mut u32) {
    *global_state += 1;
}

fn main() {
    let mut global_state = 0u32;

    change(&mut global_state);

    println!("Final state: {}", global_state);
}
