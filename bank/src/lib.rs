pub struct SavingsAccounts {
    balance: i32,
}

impl SavingsAccounts {
    fn new(balance: i32) -> Self {
        Self { balance }
    }

    //Get balance will return the balance of our account
    fn get_balance(&self) -> i32 {
        self.balance
    }

    //Deposit to add money to account
    fn deposit(&mut self, amount: u32) {
        self.balance += amount as i32;
    }

    pub fn transfer_money(&mut self, amount: i32, acc_no: u32) -> Result<String, String> {
        if self.balance >= amount {
            self.balance -= amount;
            return Ok(format!("Transferred ${amount} to {acc_no}"));
        } else {
            Err(format!("You dont have enough balance"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use super::*;

    #[test]
    //Testing if balance can have 0
    fn should_have_0() {
        let account = SavingsAccounts::new(0);
        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccounts::new(0);
        account.deposit(1000);
        assert_eq!(account.get_balance(), 100, "Balance should be 100");
        assert_ne!(account.get_balance(), 0);
        assert!(account.get_balance() == 100);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut acc = SavingsAccounts::new(100);
        acc.transfer_money(500, 123456)?;
        Ok(())
    }
}
