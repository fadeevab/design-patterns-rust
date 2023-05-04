mod freight_train;
mod passenger_train;

pub use freight_train::FreightTrain;
pub use passenger_train::PassengerTrain;

use crate::train_station::Mediator;

// A train gets a mediator object by reference.
pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}
