use std::{cell::RefMut, collections::HashMap};

use crate::models::transactions::{transactions_set, TRANSACTIONS};

use super::transaction::{Amount, ClientID, Transaction};

type Result<T> = std::result::Result<T, RejectedTransaction>;
pub type Disputes = HashMap<ClientID, Transaction>;

#[derive(Debug, Clone)]
pub struct RejectedTransaction;

pub struct Account {
    client_id: ClientID,
    available: Amount,
    held: Amount,
    locked: bool,
    disputes: Disputes,
}

impl Account {
    pub fn new(id: ClientID) -> Self {
        Self {
            client_id: id,
            available: 0f32,
            held: 0f32,
            locked: false,
            disputes: Default::default(),
        }
    }

    // A deposit is a credit to the client's asset account, meaning it should increase the available and total funds of the client account.
    pub fn process_deposit(&mut self, transaction: &Transaction) -> Result<Transaction> {
        println!("Processing DEPOSIT {:?}", transaction);
        self.available += transaction.amount;
        Ok(transaction.clone())
    }

    // A withdraw is a debit to the client's asset account, meaning it should decrease the available and total funds of the client account.
    pub fn process_withdrawal(&mut self, transaction: &Transaction) -> Result<Transaction> {
        println!("Processing WITHDRAWAL {:?}", transaction);
        if self.available > transaction.amount {
            self.available -= transaction.amount;
            Ok(transaction.clone())
        } else {
            Err(RejectedTransaction)
        }
    }

    // A dispute represents a client's claim that a transaction was erroneous and should be reversed.
    // The transaction shouldn't be reversed yet but the associated funds should be held.
    // This means that the clients available funds should decrease by the amount disputed,
    // their held funds should increase by the amount disputed,
    // while their total funds should remain the same.
    pub fn process_dispute(&mut self, transaction: &Transaction) -> Result<Transaction> {
        println!("Processing DISPUTE {:?}", transaction);
        let transactions = TRANSACTIONS
            .read()
            .expect("Could not get read access to the transactions store");
        let disputed_tx = transactions.get(transaction.id);
        match disputed_tx {
            None => {
                println!(
                    "Ignoring invalid disputed transaction ID {:?}",
                    transaction.id
                );
                Err(RejectedTransaction)
            }
            Some(tx) => {
                // Ok, but what the process should do  with a dispute that is greater than the available balance?
                // Until other clarification, I'm coding it to reject that claim.
                if self.available > tx.amount {
                  self.held += tx.amount;
                  self.available -= tx.amount;
                  Ok(transaction.clone())
                } else {
                    Err(RejectedTransaction)
                }
            }
        }
    }

    pub fn available_balance(&self) -> Amount {
        self.available
    }

    pub fn total_balance(&self) -> Amount {
        self.available + self.held
    }
}