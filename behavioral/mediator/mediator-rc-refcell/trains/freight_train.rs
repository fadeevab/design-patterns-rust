use std::{cell::RefCell, rc::Rc};

use super::Train;
use crate::train_station::Mediator;

pub struct FreightTrain {
    name: String,
    mediator: Rc<RefCell<dyn Mediator>>,
}

impl FreightTrain {
    pub fn new(name: String, mediator: Rc<RefCell<dyn Mediator>>) -> Self {
        Self { name, mediator }
    }
}

impl Train for FreightTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&self) {
        if !self.mediator.borrow().notify_about_arrival(self) {
            println!("Freight train {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Freight train {}: Arrived", self.name);
    }

    fn depart(&self) {
        println!("Freight train {}: Leaving", self.name);
        self.mediator.borrow().notify_about_departure(self);
    }
}
