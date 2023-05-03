use super::Train;
use crate::train_station::Mediator;

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!("Passenger train {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Passenger train {}: Arrived", self.name);
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Passenger train {}: Leaving", self.name);
        mediator.notify_about_departure(&self.name);
    }
}
