//! A pure safe way to implement Singleton in Rust is using no static variables
//! and passing everything around through function arguments.
//! The oldest living variable is an object created at the start of the `main()`.

fn change(global_state: &mut u32) {
    *global_state += 1;
}

fn main() {
    let mut global_state = 0u32;

    change(&mut global_state);

    println!("Final state: {}", global_state);
}
