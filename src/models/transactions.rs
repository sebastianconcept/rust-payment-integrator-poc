use std::collections::HashMap;

use super::transaction::{Transaction, TransactionID};

#[derive(Debug, Clone)]
pub struct Transactions {
    pub store: HashMap<TransactionID, Transaction>,
}

impl Transactions {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }

    pub fn set(&mut self, transaction: Transaction) -> Option<TransactionID> {
        let txid = transaction.id;
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


