#[derive(PartialEq, Debug)]
pub enum Transaction {
    Deposit(u32),
    Withdraw(u32)
}

pub trait TransactionRepository {
    fn add(&mut self, transaction: Transaction);
}
