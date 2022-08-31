use std::collections::HashMap;

use crate::app::TRANSACTIONS;

use super::transaction::{Transaction, TransactionID};

pub struct Transactions {
    pub store: HashMap<TransactionID, Transaction>,
}

impl Transactions {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }

    pub fn set(&mut self, txid: TransactionID, transaction: Transaction) -> Option<TransactionID> {
        self.store.insert(txid, transaction);
        Some(txid)
    }

    pub fn get(&self, txid: TransactionID) -> Option<&Transaction> {
        self.store.get(&txid)
    }

    pub fn get_mut(&mut self, txid: TransactionID) -> Option<&mut Transaction> {
        self.store.get_mut(&txid)
    }

    pub fn size(&self) -> usize {
        self.store.len()
    }

    pub fn reset(&mut self) {
        self.store.clear();
    }
}

pub fn transactions_size() -> usize {
    TRANSACTIONS
        .read()
        .expect("Could not get read access to the transactions store")
        .size()
}

pub fn transactions_set(transaction: Transaction) {
    TRANSACTIONS
        .write()
        .expect("Could not get write access to the transactions store")
        .set(transaction.id, transaction);
}
