use std::collections::HashMap;

use super::transaction::{Amount, ClientID, Transaction};

pub type Disputes = HashMap<ClientID, Transaction>;

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

    pub fn available_balance(&self) -> Amount {
        self.available
    }

    pub fn total_balance(&self) -> Amount {
        self.available + self.held
    }
}
