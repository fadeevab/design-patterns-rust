//! `OnceCell` allows having a custom initialization of a singleton at
//! an arbitrary place. The initialization can be done only once.
//! `Mutex` is still needed to make an actual object modifiable
//! without an `unsafe` block.

use once_cell::sync::OnceCell;
use std::sync::Mutex;

static ARRAY: OnceCell<Mutex<Vec<i32>>> = OnceCell::new();

fn singleton_init(array: Vec<i32>) {
    ARRAY.get_or_init(|| Mutex::new(array));
}

fn do_a_call() {
    ARRAY.get().unwrap().lock().unwrap().push(1);
}

fn main() {
    singleton_init(vec![42]);

    do_a_call();
    do_a_call();
    do_a_call();

    println!("{:?}", ARRAY.get().unwrap().lock().unwrap());
}
