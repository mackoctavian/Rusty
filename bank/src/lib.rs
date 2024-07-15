pub struct SavingsAccounts {
    balance: i32,
}

impl SavingsAccounts {
    pub fn new(balance: i32) -> Self {
        Self { balance }
    }

    //Get balance will return the balance of our account
    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    //Deposit to add money to account
    pub fn deposit(&mut self, amount: u32) {
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
