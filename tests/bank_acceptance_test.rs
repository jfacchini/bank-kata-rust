use bank::account::{Account, Transaction, TransactionRepository};

struct Console {
    buffer: String
}
impl Console {
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
}

struct Repository;
impl TransactionRepository for Repository {
    fn add(&self, transaction: Transaction) {
        todo!()
    }
}

#[test]
fn print_statement_lists_the_account_transactions_in_reverse_chronological_order() {
    let console = Console::new();
    let repository = Repository;
    let account = Account::new(repository);

    account.deposit(1000);
    account.deposit(2000);
    account.withdraw(500);
    account.print_statement();

    assert_eq!(console.buffer, format!(
        "{}\n{}\n{}\n{}\n",
        "Date || Amount || Balance",
        "14/01/2012 || -500 || 2500",
        "13/01/2012 || 2000 || 3000",
        "10/01/2012 || 1000 || 1000"
    ))
}
