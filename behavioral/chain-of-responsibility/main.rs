mod department;
mod patient;

use department::{Cashier, Department, Doctor, Medical, Reception};
use patient::Patient;

fn main() {
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    // Reception handles a patient passing him to the next link in the chain.
    // Reception -> Doctor -> Medical -> Cashier.
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    reception.execute(&mut patient);
}
