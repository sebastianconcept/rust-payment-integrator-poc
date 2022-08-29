use std::{
    collections::HashMap,
};

use csv::StringRecord;

use crate::models::{
    account::Account,
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

    pub fn process(&mut self, transaction: Transaction) {
        let tid = transaction.id.clone();
        transactions_set(transaction);
        let transactions = TRANSACTIONS
            .read()
            .expect("Could not get read access to the transactions store");
        let tx = transactions.get(tid);
        match tx {
            None => println!("Ignoring invalid transaction id {}", tid),
            Some(txn) => {
                match txn.kind {
                    TransactionType::Deposit => self.process_deposit(txn),
                    TransactionType::Withdrawal => self.process_withdrawal(txn),
                    TransactionType::Dispute => self.process_dispute(txn),
                    TransactionType::Resolve => self.process_resolve(txn),
                    TransactionType::Chargeback => self.process_chargeback(txn),
                };
            }
        }
    }

    pub fn process_record(&mut self, record: StringRecord) {
        let transaction = Transaction::from_record(record);
        match transaction {
            Err(err) => {
                // Ignoring unexpected invalid transaction input
                println!("Ignoring invalid transaction record (unknown transaction type)")
            }
            Ok(tx) => {
                let tid = tx.id.clone();
                transactions_set(tx);
                let transactions = TRANSACTIONS
                    .read()
                    .expect("Could not get read access to the transactions store");
                let tx = transactions.get(tid);
                match tx {
                    None => println!("Ignoring invalid transaction id {}", tid),
                    Some(txn) => match txn.kind {
                        TransactionType::Deposit => self.process_deposit(txn),
                        TransactionType::Withdrawal => self.process_withdrawal(txn),
                        TransactionType::Dispute => self.process_dispute(txn),
                        TransactionType::Resolve => self.process_resolve(txn),
                        TransactionType::Chargeback => self.process_chargeback(txn),
                    },
                }
            }
        }
    }

    fn process_deposit(&mut self, transaction: &Transaction) {
        let client_id = transaction.client_id;
        let account = self.get_account(client_id);
        account.process_deposit(transaction).unwrap();
    }

    fn process_withdrawal(&mut self, transaction: &Transaction) {
        let account = self.get_account(transaction.client_id);
        account.process_withdrawal(transaction).unwrap();
    }

    fn process_dispute(&mut self, transaction: &Transaction) {
        let account = self.get_account(transaction.client_id);
        account.process_dispute(transaction).unwrap();
    }

    fn process_resolve(&mut self, transaction: &Transaction) {
        println!("Processing RESOLVE {:?}", transaction)
    }

    fn process_chargeback(&mut self, transaction: &Transaction) {
        println!("Processing CHARGEBACK {:?}", transaction)
    }

    pub fn get_available_balance(&mut self, client_id: ClientID) -> Amount {
        let account = self.get_account(client_id);
        account.available_balance()
    }
}
