#[derive(Default)]
pub struct Patient {
    pub name: String,
    pub registration_done: bool,
    pub doctor_check_up_done: bool,
    pub medicine_done: bool,
    pub payment_done: bool,
}
