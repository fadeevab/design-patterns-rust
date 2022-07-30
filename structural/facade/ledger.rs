pub struct Ledger;

impl Ledger {
    pub fn make_entry(&mut self, account_id: &String, txn_type: String, amount: u32) {
        println!(
            "Make ledger entry for accountId {} with transaction type {} for amount {}",
            account_id, txn_type, amount
        );
    }
}
