pub struct Notification;

impl Notification {
    pub fn send_wallet_credit_notification(&self) {
        println!("Sending wallet credit notification");
    }

    pub fn send_wallet_debit_notification(&self) {
        println!("Sending wallet debit notification");
    }
}
