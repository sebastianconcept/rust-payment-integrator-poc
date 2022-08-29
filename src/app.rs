use std::{collections::HashMap};

use csv::StringRecord;

use crate::models::{
    account::{Account, Disputes},
    transaction::{Amount, ClientID, Transaction, TransactionType},
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

    pub fn process(&self, transaction: Transaction) {
        match transaction.kind {
            TransactionType::Deposit => self.process_deposit(transaction),
            TransactionType::Withdrawal => self.process_withdrawal(transaction),
            TransactionType::Dispute => self.process_dispute(transaction),
            TransactionType::Resolve => self.process_resolve(transaction),
            TransactionType::Chargeback => self.process_chargeback(transaction),
        };
    }

    pub fn process_record(&self, record: StringRecord) {
        let transaction = Transaction::from_record(record);
        match transaction {
            Err(err) => {
                // Ignoring unexpected invalid transaction input
                println!("Ignoring invalid transaction record (unknown transaction type)")
            }
            Ok(tx) => match tx.kind {
                TransactionType::Deposit => self.process_deposit(tx),
                TransactionType::Withdrawal => self.process_withdrawal(tx),
                TransactionType::Dispute => self.process_dispute(tx),
                TransactionType::Resolve => self.process_resolve(tx),
                TransactionType::Chargeback => self.process_chargeback(tx),
            },
        }
    }

    fn process_deposit(&self, transaction: Transaction) {
        println!("Processing DEPOSIT {:?}", transaction)
    }

    fn process_withdrawal(&self, transaction: Transaction) {
        println!("Processing WITHDRAWAL {:?}", transaction)
    }

    fn process_dispute(&self, transaction: Transaction) {
        println!("Processing DISPUTE {:?}", transaction)
    }

    fn process_resolve(&self, transaction: Transaction) {
        println!("Processing RESOLVE {:?}", transaction)
    }

    fn process_chargeback(&self, transaction: Transaction) {
        println!("Processing CHARGEBACK {:?}", transaction)
    }

    pub fn get_available_balance(&mut self, client_id: ClientID) -> Amount {
        let account = self.get_account(client_id);
        account.available_balance()
    }
}
