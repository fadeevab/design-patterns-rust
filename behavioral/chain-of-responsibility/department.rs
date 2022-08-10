mod cashier;
mod doctor;
mod medical;
mod reception;

pub use cashier::Cashier;
pub use doctor::Doctor;
pub use medical::Medical;
pub use reception::Reception;

use crate::patient::Patient;

/// A single role of objects that make up a chain.
/// A typical trait implementation must have `handle` and `next` methods,
/// while `execute` is implemented by default and contains a proper chaining
/// logic.
pub trait Department {
    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);

        if let Some(next) = &mut self.next() {
            next.execute(patient);
        }
    }

    fn handle(&mut self, patient: &mut Patient);
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;
}

/// Helps to wrap an object into a boxed type.
pub(self) fn into_next(
    department: impl Department + Sized + 'static,
) -> Option<Box<dyn Department>> {
    Some(Box::new(department))
}
