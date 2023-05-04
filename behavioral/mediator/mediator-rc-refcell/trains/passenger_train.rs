use std::{cell::RefCell, rc::Rc};

use super::Train;
use crate::train_station::Mediator;

pub struct PassengerTrain {
    name: String,
    mediator: Rc<RefCell<dyn Mediator>>,
}

impl PassengerTrain {
    pub fn new(name: String, mediator: Rc<RefCell<dyn Mediator>>) -> Self {
        Self { name, mediator }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&self) {
        if !self.mediator.borrow().notify_about_arrival(self) {
            println!("Passenger train {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Passenger train {}: Arrived", self.name);
    }

    fn depart(&self) {
        println!("Passenger train {}: Leaving", self.name);
        self.mediator.borrow().notify_about_departure(self);
    }
}
