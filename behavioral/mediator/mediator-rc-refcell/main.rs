mod train_station;
mod trains;

use std::{cell::RefCell, rc::Rc};

use train_station::StationManager;
use trains::{FreightTrain, PassengerTrain, Train};

fn main() {
    let station = Rc::new(RefCell::new(StationManager::default()));

    let train1 = Rc::new(PassengerTrain::new("Train 1".into(), station.clone()));
    let train2 = Rc::new(FreightTrain::new("Train 2".into(), station.clone()));

    {
        let mut station = station.borrow_mut();
        station.register(train1.clone());
        station.register(train2.clone());
    }

    train1.arrive();
    train2.arrive();
    train1.depart();
    train2.depart();
}
