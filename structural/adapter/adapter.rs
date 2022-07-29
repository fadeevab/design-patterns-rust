use crate::{adaptee::Adaptee, Target};

pub struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    pub fn new(adaptee: Adaptee) -> Self {
        Self { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        self.adaptee.specific_request().chars().rev().collect()
    }
}
