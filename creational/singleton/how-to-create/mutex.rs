//! ructc 1.63
//! https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//!
//! Starting with Rust 1.63, it can be easier to work with global mutable
//! singletons.
//!
//! Now that `Mutex::new` is `const`, you can use global static `Mutex` locks
//! without needing lazy initialization.

use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    let array = ARRAY.lock().unwrap();
    println!("Called {} times: {:?}", array.len(), array);
    drop(array);

    *ARRAY.lock().unwrap() = vec![3, 4, 5];

    println!("New singleton object: {:?}", ARRAY.lock().unwrap());
}
