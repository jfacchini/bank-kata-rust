mod transaction;

use std::cell::RefCell;
use std::rc::Rc;
pub use transaction::{Transaction, TransactionRepository};

pub struct Account<T: TransactionRepository> {
    transaction_repository: Rc<RefCell<T>>
}

impl<T: TransactionRepository> Account<T> {
    pub fn deposit(&self, amount: u32) {
        let mut repository = self.transaction_repository.as_ref().borrow_mut();
        repository.add(Transaction::Deposit(amount))
    }
    pub fn withdraw(&self, amount: u32) {
        todo!()
    }
    pub fn print_statement(&self) {
        todo!()
    }

    pub fn new(transaction_repository: Rc<RefCell<T>>) -> Self {
        Self {
            transaction_repository
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Transaction::Deposit;
    use super::*;

    struct InMemoryRepository<T> {
        values: Vec<T>
    }
    impl<T> InMemoryRepository<T> {
        fn new() -> Self {
            Self {
                values: Vec::new()
            }
        }
    }
    impl TransactionRepository for InMemoryRepository<Transaction> {
        fn add(&mut self, transaction: Transaction) {
            self.values.push(transaction);
        }
    }

    #[test]
    fn add_a_new_deposit_transaction_in_repository() {
        let repository = Rc::new(RefCell::new(InMemoryRepository::new()));
        let account = Account::new(Rc::clone(&repository));
        let account2 = Account::new(Rc::clone(&repository));

        account.deposit(123);
        account2.deposit(432);
        account.deposit(67567);

        assert_eq!(repository.borrow().values, vec![
            Deposit(123),
            Deposit(432),
            Deposit(67567)
        ]);
    }
}
