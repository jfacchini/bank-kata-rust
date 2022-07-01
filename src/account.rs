mod transaction;

pub use transaction::{Transaction, TransactionRepository};

pub struct Account<T: TransactionRepository> {
    transaction_repository: T
}

impl<T: TransactionRepository> Account<T> {
    pub fn deposit(&self, amount: u32) {
        self.transaction_repository.add(Transaction::Deposit(amount))
    }
    pub fn withdraw(&self, amount: u32) {
        todo!()
    }
    pub fn print_statement(&self) {
        todo!()
    }

    pub fn new(transaction_repository: T) -> Self {
        Self {
            transaction_repository
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::account::Transaction::Deposit;
    use super::*;

    struct InMemoryRepository<T> {
        values: Rc<RefCell<Vec<T>>>
    }
    impl<T> InMemoryRepository<T> {
        fn from(values: Rc<RefCell<Vec<T>>>) -> Self {
            Self {
                values
            }
        }
    }
    impl TransactionRepository for InMemoryRepository<Transaction> {
        fn add(&self, transaction: Transaction) {
            self.values.borrow_mut().push(transaction);
        }
    }

    #[test]
    fn add_a_new_deposit_transaction_in_repository() {
        let transactions = Rc::new(RefCell::new(Vec::<Transaction>::new()));
        let account = Account::new(InMemoryRepository::from(Rc::clone(&transactions)));
        let account2 = Account::new(InMemoryRepository::from(Rc::clone(&transactions)));

        account.deposit(123);
        account2.deposit(432);
        account.deposit(67567);

        assert_eq!(transactions.borrow().as_ref(), vec![
            Deposit(123),
            Deposit(432),
            Deposit(67567)
        ]);
    }
}
