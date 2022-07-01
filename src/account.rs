use crate::account::transaction::{Transaction, TransactionRepository};

pub mod transaction;

pub struct Account {

}

impl Account {
    pub fn new() -> Self {
        Self {

        }
    }
    pub fn deposit(&self, amount: u32) {
        todo!()
    }
    pub fn withdraw(&self, amount: u32) {
        todo!()
    }
    pub fn print_statement(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
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
            self.values.as_ref().borrow_mut().push(transaction);
        }
    }

    #[test]
    fn add_a_new_deposit_in_transaction_repository() {
        let transactions = Rc::new(RefCell::new(Vec::<Transaction>::new()));
        let transaction_repository = InMemoryRepository::from(Rc::clone(&transactions));

        let account = Account::new();
        account.deposit(123);

        assert_eq!(transactions.as_ref().borrow().as_ref(), vec![Transaction::Deposit(123)]);
    }
}
