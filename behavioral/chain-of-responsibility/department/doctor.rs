use super::{into_next, Department, Patient};

pub struct Doctor {
    next: Option<Box<dyn Department>>,
}

impl Doctor {
    pub fn new(next: impl Department + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Department for Doctor {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.doctor_check_up_done {
            println!("A doctor checkup is already done");
        } else {
            println!("Doctor checking a patient {}", patient.name);
            patient.doctor_check_up_done = true;
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}
