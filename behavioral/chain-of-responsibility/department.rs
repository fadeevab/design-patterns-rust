mod cashier;
mod doctor;
mod medical;
mod reception;

pub use cashier::Cashier;
pub use doctor::Doctor;
pub use medical::Medical;
pub use reception::Reception;

use crate::patient::Patient;

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

pub(self) fn into_next(
    department: impl Department + Sized + 'static,
) -> Option<Box<dyn Department>> {
    Some(Box::new(department))
}
