use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::trains::Train;

pub trait Mediator {
    fn notify_about_arrival(&self, train: &dyn Train) -> bool;
    fn notify_about_departure(&self, train: &dyn Train);
}

#[derive(Default)]
pub struct StationManager {
    trains: HashMap<String, Rc<dyn Train>>,
    train_queue: RefCell<VecDeque<String>>,
    train_on_platform: RefCell<Option<String>>,
}

impl StationManager {
    pub fn register(&mut self, train: Rc<dyn Train>) {
        self.trains.insert(train.name().clone(), train);
    }
}

impl Mediator for StationManager {
    fn notify_about_arrival(&self, train: &dyn Train) -> bool {
        let train_name = train.name().clone();

        self.trains.get(&train_name).expect("A train should exist");

        if self.train_on_platform.borrow().is_some() {
            self.train_queue.borrow_mut().push_back(train_name.clone());
            return false;
        }

        self.train_on_platform.replace(Some(train_name));
        return true;
    }

    fn notify_about_departure(&self, train: &dyn Train) {
        if Some(train.name().clone()) != self.train_on_platform.replace(None) {
            return;
        }

        let next_train = self.train_queue.borrow_mut().pop_front();

        if let Some(next_train_name) = next_train {
            let next_train = self.trains.get(&next_train_name).unwrap();
            next_train.arrive();
        }
    }
}
