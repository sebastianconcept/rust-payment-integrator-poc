use std::collections::HashMap;

use super::transaction::{Amount, ClientID, Transaction};

type Result<T> = std::result::Result<T, RejectedTransaction>;
pub type Disputes = HashMap<ClientID, Transaction>;

#[derive(Debug, Clone)]
pub struct RejectedTransaction {}

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
    pub fn process_deposit(&mut self, transaction: Transaction) -> Result<Transaction> {
      self.available += transaction.amount;
      Ok(transaction)
    }

    pub fn available_balance(&self) -> Amount {
        self.available
    }

    pub fn total_balance(&self) -> Amount {
        self.available + self.held
    }
}
