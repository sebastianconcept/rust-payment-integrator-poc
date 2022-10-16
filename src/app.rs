use std::collections::HashMap;

use csv::StringRecord;

use crate::models::{
    account::{Account, RejectedTransaction, Result},
    output::Output,
    transaction::{Amount, ClientID, Transaction, TransactionType},
    transactions::Transactions,
};

type Accounts = HashMap<ClientID, Account>;

#[derive(Debug, Clone)]
pub struct App {
    pub accounts: Accounts,
    transactions: Transactions,
    output: Output,
}

impl App {
    pub fn new() -> Self {
        Self {
            accounts: Default::default(),
            transactions: Transactions::new(),
            output: Output::new(),
        }
    }

    pub fn process(&mut self, transaction: Transaction) -> Result<Transaction> {
        if (transaction.kind == TransactionType::Deposit)
            || (transaction.kind == TransactionType::Withdrawal)
        {
            // We only need to store deposits and withdrawals
            self.transactions.set(transaction.clone());
        }
        match transaction.kind {
            TransactionType::Deposit => Self::process_deposit(&mut self.accounts, &transaction),
            TransactionType::Withdrawal => {
                Self::process_withdrawal(&mut self.accounts, &transaction)
            }
            TransactionType::Dispute => {
                Self::process_dispute(&mut self.accounts, &self.transactions, &transaction)
            }
            TransactionType::Resolve => {
                Self::process_resolve(&mut self.accounts, &self.transactions, &transaction)
            }
            TransactionType::Chargeback => {
                Self::process_chargeback(&mut self.accounts, &self.transactions, &transaction)
            }
        }
    }

    pub fn process_record(&mut self, record: StringRecord) -> Result<Transaction> {
        let transaction = Transaction::from_record(record);
        match transaction {
            Err(err) => Err(err),
            Ok(tx) => self.process(tx),
        }
    }

    fn process_deposit(accounts: &mut Accounts, transaction: &Transaction) -> Result<Transaction> {
        let account = Self::get_or_create_account(accounts, transaction.client_id);
        account.process_deposit(transaction)
    }

    fn process_withdrawal(
        accounts: &mut Accounts,
        transaction: &Transaction,
    ) -> Result<Transaction> {
        let account = Self::get_or_create_account(accounts, transaction.client_id);
        account.process_withdrawal(transaction)
    }

    fn process_dispute(
        accounts: &mut Accounts,
        transactions: &Transactions,
        transaction: &Transaction,
    ) -> Result<Transaction> {
        let account = Self::get_or_create_account(accounts, transaction.client_id);
        account.process_dispute(transaction, &transactions)
    }

    fn process_resolve(
        accounts: &mut Accounts,
        transactions: &Transactions,
        transaction: &Transaction,
    ) -> Result<Transaction> {
        let account = Self::get_or_create_account(accounts, transaction.client_id);
        account.process_resolve(transaction, &transactions)
    }

    fn process_chargeback(
        accounts: &mut Accounts,
        transactions: &Transactions,
        transaction: &Transaction,
    ) -> Result<Transaction> {
        let account = Self::get_or_create_account(accounts, transaction.client_id);
        account.process_chargeback(transaction, &transactions)
    }

    pub fn get_available_balance(&mut self, client_id: ClientID) -> Amount {
        let account = Self::get_or_create_account(&mut self.accounts, client_id);
        account.available_balance()
    }

    pub fn get_held_balance(&mut self, client_id: ClientID) -> Amount {
        let account = Self::get_or_create_account(&mut self.accounts, client_id);
        account.held_balance()
    }

    pub fn get_total_balance(&mut self, client_id: ClientID) -> Amount {
        let account = Self::get_or_create_account(&mut self.accounts, client_id);
        account.total_balance()
    }

    pub fn is_locked(&mut self, client_id: ClientID) -> bool {
        let account = Self::get_or_create_account(&mut self.accounts, client_id);
        account.is_locked()
    }

    pub fn transactions_size(&self) -> usize {
        self.transactions.size()
    }

    pub fn output_write(&self, msg: String) {
        self.output.write(msg);
    }

    fn get_or_create_account(accounts: &mut Accounts, client_id: ClientID) -> &mut Account {
        accounts
            .entry(client_id)
            .or_insert_with(|| Account::new(client_id))
    }

    pub fn get_account(&self, client_id: ClientID) -> Result<&Account> {
        self.accounts
            .get(&client_id)
            .ok_or_else(|| RejectedTransaction::IDNotFound)
    }
}

#[cfg(test)]
mod tests {
    use csv::StringRecord;
    use fraction::Decimal;

    use crate::{app::App, models::transaction::Transaction};

    #[test]
    fn deposit_can_increase_account_balance() {
        let mut app = App::new();
        let record = StringRecord::from(vec!["deposit", "    2", "5      ", " 3.0 "]);
        let tx = Transaction::from_record(record);
        match tx {
            Err(_err) => assert!(false),
            Ok(tx) => {
                let client_id = tx.client_id.clone();
                let before = app.get_available_balance(client_id);
                assert_eq!(before, Decimal::from(0));
                app.process(tx).unwrap();
                let after = app.get_available_balance(client_id);
                assert_ne!(before, after);
                assert_eq!(after, Decimal::from(3.0));
            }
        }
        let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "15", "1.5"]));
        let client_id = tx2.as_ref().unwrap().client_id;
        app.process(tx2.unwrap()).unwrap();
        let after2 = app.get_available_balance(client_id);
        assert_eq!(after2, Decimal::from(4.5));
        assert_eq!(
            app.get_held_balance(client_id) + app.get_available_balance(client_id),
            app.get_total_balance(client_id)
        );
        assert_eq!(app.is_locked(client_id), false);
    }

    #[test]
    fn withdrawal_can_decrease_account_balance() {
        let mut app = App::new();
        let record = StringRecord::from(vec!["deposit", "    2", "5      ", " 3.0 "]);
        let tx = Transaction::from_record(record);
        match tx {
            Err(_err) => assert!(false),
            Ok(tx) => {
                let client_id = tx.client_id.clone();
                app.process(tx).unwrap();
                let after = app.get_available_balance(client_id);
                assert_eq!(after, Decimal::from(3.0));
            }
        }
        let tx2 =
            Transaction::from_record(StringRecord::from(vec!["withdrawal", "2", "15", "1.3"]));
        let client_id = tx2.as_ref().unwrap().client_id;
        app.process(tx2.unwrap()).unwrap();
        let after2 = app.get_available_balance(client_id);
        assert_eq!(after2, Decimal::from(3.0 - 1.3));
        let size = app.transactions_size();
        assert_eq!(size, 2);
        assert_eq!(
            app.get_held_balance(client_id) + app.get_available_balance(client_id),
            app.get_total_balance(client_id)
        );
        assert_eq!(app.is_locked(client_id), false);
    }

    #[test]
    fn dispute_increase_disputed_balance_and_maintain_total() {
        let mut app = App::new();
        let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
        let client_id = tx1.as_ref().unwrap().client_id;
        app.process(tx1.unwrap()).unwrap();
        let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
        app.process(tx2.unwrap()).unwrap();
        let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "4", ""]));
        app.process(tx3.unwrap()).unwrap();
        let a = app.clone();
        let account = a.get_account(client_id).unwrap();
        let total = account.total_balance();
        let available = account.available_balance();
        assert_eq!(available, Decimal::from(total - Decimal::from(2.0)));
        assert_eq!(
            app.get_held_balance(client_id) + app.get_available_balance(client_id),
            app.get_total_balance(client_id)
        );
        assert_eq!(app.is_locked(client_id), false);
    }

    #[test]
    fn resolve_decrease_held_balances_increase_available_and_maintain_total() {
        let mut app = App::new();
        let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
        let client_id = tx1.as_ref().unwrap().client_id;
        app.process(tx1.unwrap()).unwrap();
        let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
        app.process(tx2.unwrap()).unwrap();
        let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "4", ""]));
        app.process(tx3.unwrap()).unwrap();
        let held_before = app.get_account(client_id).unwrap().held_balance().clone();
        let total_before = app.get_account(client_id).unwrap().total_balance().clone();
        assert_ne!(held_before, Decimal::from(0));
        assert_eq!(held_before, Decimal::from(2.0));
        assert_eq!(total_before, Decimal::from(3.5));
        let tx4 = Transaction::from_record(StringRecord::from(vec!["resolve", "2", "4", ""]));
        app.process(tx4.unwrap()).unwrap();
        let held_after = app.get_account(client_id).unwrap().held_balance().clone();
        let total_after = app.get_account(client_id).unwrap().total_balance().clone();
        assert_ne!(held_after, Decimal::from(2.0));
        assert_eq!(held_after, Decimal::from(0));
        assert_eq!(total_after, Decimal::from(3.5));
        assert_eq!(
            app.get_held_balance(client_id) + app.get_available_balance(client_id),
            app.get_total_balance(client_id)
        );
        assert_eq!(app.is_locked(client_id), false);
    }

    #[test]
    fn chargeback_decreases_held_and_total_balances_and_locks_account() {
        let mut app = App::new();
        let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
        let client_id = tx1.as_ref().unwrap().client_id;
        app.process(tx1.unwrap()).unwrap();
        let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
        app.process(tx2.unwrap()).unwrap();
        let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "4", ""]));
        app.process(tx3.unwrap()).unwrap();
        let held_before = app.get_account(client_id).unwrap().held_balance().clone();
        let total_before = app.get_account(client_id).unwrap().total_balance().clone();
        assert_ne!(held_before, Decimal::from(0));
        assert_eq!(held_before, Decimal::from(2.0));
        assert_eq!(total_before, Decimal::from(3.5));
        let tx4 = Transaction::from_record(StringRecord::from(vec!["chargeback", "2", "4", ""]));
        app.process(tx4.unwrap()).unwrap();
        let held_after = app.get_account(client_id).unwrap().held_balance().clone();
        let total_after = app.get_account(client_id).unwrap().total_balance().clone();
        assert!(app.get_account(client_id).unwrap().is_locked());
        assert_ne!(held_after, Decimal::from(2));
        assert_eq!(held_after, Decimal::from(0));
        assert_eq!(total_after, Decimal::from(1.5));
        assert_eq!(
            app.get_held_balance(client_id) + app.get_available_balance(client_id),
            app.get_total_balance(client_id)
        );
        assert_eq!(app.is_locked(client_id), true);
    }
}
