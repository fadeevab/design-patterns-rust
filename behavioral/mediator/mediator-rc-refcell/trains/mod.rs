mod freight_train;
mod passenger_train;

pub use freight_train::FreightTrain;
pub use passenger_train::PassengerTrain;

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&self);
    fn depart(&self);
}
