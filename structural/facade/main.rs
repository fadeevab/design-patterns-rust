mod account;
mod ledger;
mod notification;
mod security_code;
mod wallet;
mod wallet_facade;

use wallet_facade::WalletFacade;

fn main() -> Result<(), String> {
    let mut wallet = WalletFacade::new("abc".into(), 1234);
    println!();

    // Wallet Facade interacts with the account, code, wallet, notification and
    // ledger behind the scenes.
    wallet.add_money_to_wallet(&"abc".into(), 1234, 10)?;
    println!();

    wallet.deduct_money_from_wallet(&"abc".into(), 1234, 5)
}
