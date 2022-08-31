use std::collections::HashMap;

use csv::StringRecord;

use crate::models::{
    account::{Account, RejectedTransaction, Result},
    transaction::{Amount, ClientID, Transaction, TransactionType},
    transactions::{transactions_set, TRANSACTIONS},
};

pub struct App {
    accounts: HashMap<ClientID, Account>,
}

impl App {
    pub fn new() -> Self {
        Self {
            accounts: Default::default(),
        }
    }

    // Returns an ensured Account for the given ID.
    pub fn get_account(&mut self, client_id: ClientID) -> &mut Account {
        self.accounts
            .entry(client_id)
            .or_insert_with(|| Account::new(client_id))
    }

    pub fn process(&mut self, transaction: Transaction) -> Result<Transaction> {
        let tx;
        let transactions;
        if (transaction.kind == TransactionType::Dispute)
            || (transaction.kind == TransactionType::Resolve)
            || (transaction.kind == TransactionType::Chargeback)
        {
            tx = Some(&transaction);
        } else {
            // We only need to store deposits and withdrawals
            let tid = transaction.id.clone();
            transactions_set(transaction);
            transactions = TRANSACTIONS
                .read()
                .expect("Could not get read access to the transactions store");
            tx = transactions.get(tid);
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

    fn process_deposit(&mut self, transaction: &Transaction) -> Result<Transaction> {
        let account = self.get_account(transaction.client_id);
        account.process_deposit(transaction)
    }

    fn process_withdrawal(&mut self, transaction: &Transaction) -> Result<Transaction> {
        let account = self.get_account(transaction.client_id);
        account.process_withdrawal(transaction)
    }

    fn process_dispute(&mut self, transaction: &Transaction) -> Result<Transaction> {
        let account = self.get_account(transaction.client_id);
        account.process_dispute(transaction)
    }

    fn process_resolve(&mut self, transaction: &Transaction) -> Result<Transaction> {
        let account = self.get_account(transaction.client_id);
        account.process_resolve(transaction)
    }

    fn process_chargeback(&mut self, transaction: &Transaction) -> Result<Transaction> {
        let account = self.get_account(transaction.client_id);
        account.process_chargeback(transaction)
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
}
