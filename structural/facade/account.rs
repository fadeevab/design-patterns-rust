pub struct Account {
    name: String,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn check(&self, name: &String) -> Result<(), String> {
        if &self.name != name {
            return Err("Account name is incorrect".into());
        }

        println!("Account verified");
        Ok(())
    }
}

// func (a *Account) checkAccount(accountName string) error {
//     if a.name != accountName {
//         return fmt.Errorf("Account Name is incorrect")
//     }
//     fmt.Println("Account Verified")
//     return nil
// }
