pub struct Wallet {
    balance: u32,
}

impl Wallet {
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    pub fn credit_balance(&mut self, amount: u32) {
        self.balance += amount;
    }

    pub fn debit_balance(&mut self, amount: u32) {
        self.balance
            .checked_sub(amount)
            .expect("Balance is not sufficient");
    }
}
