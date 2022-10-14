use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
};

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
    accounts: RefCell<Accounts>,
    transactions: RefCell<Transactions>,
    output: Output,
}

impl App {
    pub fn new() -> Self {
        Self {
            accounts: RefCell::new(Default::default()),
            transactions: RefCell::new(Transactions::new()),
            output: Output::new(),
        }
    }

    // Returns an ensured Account for the given ID.
    pub fn get_account(&self, client_id: ClientID) -> RefMut<'_, Account> {
        let accounts = self.accounts.borrow_mut();
        RefMut::map(accounts, |hashmap| {
            hashmap
                .entry(client_id)
                .or_insert_with(|| Account::new(client_id))
        })
    }

    pub fn process(&self, transaction: Transaction) -> Result<Transaction> {
        let txns;
        let tx;
        if (transaction.kind == TransactionType::Dispute)
            || (transaction.kind == TransactionType::Resolve)
            || (transaction.kind == TransactionType::Chargeback)
        {
            tx = Some(&transaction);
        } else {
            // We only need to store deposits and withdrawals
            let tid = transaction.id.clone();
            self.transactions.borrow_mut().set(transaction);
            txns = self.transactions.borrow();
            tx = txns.get(tid);
        }
        match tx {
            None => Err(RejectedTransaction::IDNotFound),
            Some(txn) => match txn.kind {
                TransactionType::Deposit => self.process_deposit(txn),
                TransactionType::Withdrawal => self.process_withdrawal(txn),
                TransactionType::Dispute => self.process_dispute(txn),
                TransactionType::Resolve => self.process_resolve(txn),
                TransactionType::Chargeback => self.process_chargeback(txn),
            },
        }
    }

    pub fn process_record(&mut self, record: StringRecord) -> Result<Transaction> {
        let transaction = Transaction::from_record(record);
        match transaction {
            Err(err) => Err(err),
            Ok(tx) => self.process(tx),
        }
    }

    fn process_deposit(&self, transaction: &Transaction) -> Result<Transaction> {
        let mut account = self.get_account(transaction.client_id);
        account.process_deposit(transaction)
    }

    fn process_withdrawal(&self, transaction: &Transaction) -> Result<Transaction> {
        let mut account = self.get_account(transaction.client_id);
        account.process_withdrawal(transaction)
    }

    fn process_dispute(&self, transaction: &Transaction) -> Result<Transaction> {
        let mut account = self.get_account(transaction.client_id);
        account.process_dispute(transaction, &mut self.transactions.borrow_mut())
    }

    fn process_resolve(&self, transaction: &Transaction) -> Result<Transaction> {
        let mut account = self.get_account(transaction.client_id);
        account.process_resolve(transaction, &mut self.transactions.borrow_mut())
    }

    fn process_chargeback(&self, transaction: &Transaction) -> Result<Transaction> {
        let mut account = self.get_account(transaction.client_id);
        account.process_chargeback(transaction, &mut self.transactions.borrow_mut())
    }

    pub fn get_available_balance(&mut self, client_id: ClientID) -> Amount {
        let account = self.get_account(client_id);
        account.available_balance()
    }

    pub fn get_held_balance(&mut self, client_id: ClientID) -> Amount {
        let account = self.get_account(client_id);
        account.held_balance()
    }

    pub fn get_total_balance(&mut self, client_id: ClientID) -> Amount {
        let account = self.get_account(client_id);
        account.total_balance()
    }

    pub fn is_locked(&mut self, client_id: ClientID) -> bool {
        let account = self.get_account(client_id);
        account.is_locked()
    }

    pub fn transactions_size(&self) -> usize {
        self.transactions.borrow().size()
    }

    pub fn output_write(&self, msg: String) {
        self.output.write(msg);
    }
}
