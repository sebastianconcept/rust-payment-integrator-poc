use std::{sync::RwLock, collections::HashMap};

use csv::StringRecord;

use crate::models::{transaction::{Transaction, TransactionType}, *, commands::{deposit::Deposit, dispute::Dispute}};

pub type Disputes = RwLock<HashMap<u16,Dispute>>;

pub struct App {
    // accounts: RwLock<HashMap<u16,Account>>
    disputes: Disputes,
    rules: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self { 
            rules: Vec::new(),
            disputes: Default::default(),
         }
    }

    pub fn process(&self, transaction: StringRecord) {
        println!("Processing {:?}", transaction)
    }

    pub fn process_record(&self, record: StringRecord) {
        let transaction = Transaction::from_record(record);
        match transaction {
            Err(err) => {
                // Ignoring unexpected invalid transaction input
                println!("Ignoring invalid transaction record (unknown transaction type")
            },
            Ok(tx) => {
                match tx.kind {
                    TransactionType::Deposit => self.process_deposit(tx),
                    TransactionType::Withdrawal => self.process_withdrawal(tx),
                    TransactionType::Dispute => self.process_dispute(tx),
                    TransactionType::Resolve => self.process_resolve(tx),
                    TransactionType::Chargeback => self.process_chargeback(tx),
                    
                }
            }
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
}
