use crate::{
    account::Account, ledger::Ledger, notification::Notification, security_code::SecurityCode,
    wallet::Wallet,
};

/// Facade hides a complex logic behind the API.
pub struct WalletFacade {
    account: Account,
    wallet: Wallet,
    code: SecurityCode,
    notification: Notification,
    ledger: Ledger,
}

impl WalletFacade {
    pub fn new(account_id: String, code: u32) -> Self {
        println!("Starting create account");

        let this = Self {
            account: Account::new(account_id),
            wallet: Wallet::new(),
            code: SecurityCode::new(code),
            notification: Notification,
            ledger: Ledger,
        };

        println!("Account created");
        this
    }

    pub fn add_money_to_wallet(
        &mut self,
        account_id: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting add money to wallet");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.credit_balance(amount);
        self.notification.send_wallet_credit_notification();
        self.ledger.make_entry(account_id, "credit".into(), amount);
        Ok(())
    }

    pub fn deduct_money_from_wallet(
        &mut self,
        account_id: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting debit money from wallet");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.debit_balance(amount);
        self.notification.send_wallet_debit_notification();
        self.ledger.make_entry(account_id, "debit".into(), amount);
        Ok(())
    }
}
