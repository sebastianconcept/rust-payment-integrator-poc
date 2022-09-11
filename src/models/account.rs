use std::collections::HashMap;

use fraction::Decimal;

use super::{
    transaction::{Amount, ClientID, Transaction},
    transactions::{self, Transactions},
};

pub type Result<T> = std::result::Result<T, RejectedTransaction>;
pub type Disputes = HashMap<ClientID, Transaction>;

#[derive(Debug, Clone)]
pub enum RejectedTransaction {
    InvalidType,
    InsufficientFunds,
    IDNotFound,
    InconsistentWithValueHeld,
    InvalidInput,
    TargetTransactionAmountMissing,
    AccountLocked,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub client_id: ClientID,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
}

impl Account {
    pub fn new(id: ClientID) -> Self {
        Self {
            client_id: id,
            available: Decimal::from(0),
            held: Decimal::from(0),
            total: Decimal::from(0),
            locked: false,
        }
    }

    // A deposit is a credit to the client's asset account, meaning it should increase the available and total funds of the client account.
    pub fn process_deposit(&mut self, transaction: &Transaction) -> Result<Transaction> {
        if self.locked {
            return Err(RejectedTransaction::AccountLocked);
        };
        let amount;
        match transaction.amount {
            None => return Err(RejectedTransaction::TargetTransactionAmountMissing),
            Some(value) => amount = value,
        };
        self.available += amount;
        self.total += amount;
        Ok(transaction.clone())
    }

    // A withdraw is a debit to the client's asset account, meaning it should decrease the available and total funds of the client account.
    pub fn process_withdrawal(&mut self, transaction: &Transaction) -> Result<Transaction> {
        if self.locked {
            return Err(RejectedTransaction::AccountLocked);
        };
        let amount;
        match transaction.amount {
            None => return Err(RejectedTransaction::TargetTransactionAmountMissing),
            Some(value) => amount = value,
        };
        if self.available > amount {
            self.available -= amount;
            self.total -= amount;
            Ok(transaction.clone())
        } else {
            Err(RejectedTransaction::InsufficientFunds)
        }
    }

    // A dispute represents a client's claim that a transaction was erroneous and should be reversed.
    // The transaction shouldn't be reversed yet but the associated funds should be held.
    // This means that the clients available funds should decrease by the amount disputed,
    // their held funds should increase by the amount disputed,
    // while their total funds should remain the same.
    pub fn process_dispute(
        &mut self,
        transaction: &Transaction,
        transactions: &mut Transactions,
    ) -> Result<Transaction> {
        if self.locked {
            return Err(RejectedTransaction::AccountLocked);
        };
        let disputed_tx = transactions.get(transaction.id);
        match disputed_tx {
            None => {
                // Ignoring invalid disputed transaction ID
                Err(RejectedTransaction::IDNotFound)
            }
            Some(tx) => {
                let amount;
                match tx.amount {
                    None => return Err(RejectedTransaction::TargetTransactionAmountMissing),
                    Some(value) => amount = value,
                }
                // Ok, but what the process should do with a dispute that is greater than the available balance?
                // Until other clarification, I'm coding it to reject that claim.
                if self.available > amount {
                    self.held += amount;
                    self.available -= amount;
                    Ok(transaction.clone())
                } else {
                    Err(RejectedTransaction::InsufficientFunds)
                }
            }
        }
    }

    // A resolve represents a resolution to a dispute, releasing the associated held funds.
    // Funds that were previously disputed are no longer disputed.
    // This means that the clients held funds should decrease by the amount no longer disputed,
    // their available funds should increase by the amount no longer disputed,
    // and their total funds should remain the same.
    pub fn process_resolve(
        &mut self,
        transaction: &Transaction,
        transactions: &mut Transactions,
    ) -> Result<Transaction> {
        if self.locked {
            return Err(RejectedTransaction::AccountLocked);
        };
        let resolved_tx = transactions.get(transaction.id);
        match resolved_tx {
            None => {
                // Ignoring invalid resolved transaction ID
                Err(RejectedTransaction::IDNotFound)
            }
            Some(tx) => {
                let amount;
                match tx.amount {
                    None => return Err(RejectedTransaction::TargetTransactionAmountMissing),
                    Some(value) => amount = value,
                }
                // Ok, but what the process should do with a resolve that has a greater amount value than the held balance?
                // Until other clarification, I'm coding it to reject that resolution.
                if amount > self.held {
                    // This means there is a transaction value inconsistency?
                    // Some kind of warning should be triggered for someone to supervise?
                    // Rejecting this resolve transaction to evade potential mistakes on account balances.
                    Err(RejectedTransaction::InconsistentWithValueHeld)
                } else {
                    self.held -= amount;
                    self.available += amount;
                    Ok(transaction.clone())
                }
            }
        }
    }

    // A chargeback is the final state of a dispute and represents the client reversing a transaction.
    // Funds that were held have now been withdrawn.
    // This means that the clients held funds and total funds should decrease by the amount previously disputed.
    // If a chargeback occurs the client's account should be immediately frozen.
    pub fn process_chargeback(
        &mut self,
        transaction: &Transaction,
        transactions: &mut Transactions,
    ) -> Result<Transaction> {
        if self.locked {
            return Err(RejectedTransaction::AccountLocked);
        };
        let disputed_tx = transactions.get(transaction.id);
        match disputed_tx {
            None => {
                // Ignoring invalid chargeback transaction ID
                Err(RejectedTransaction::IDNotFound)
            }
            Some(tx) => {
                let amount;
                match tx.amount {
                    None => return Err(RejectedTransaction::TargetTransactionAmountMissing),
                    Some(value) => amount = value,
                }

                // What the integrator should do when there are insufficient funds for a chargeback?
                if amount > self.held {
                    return Err(RejectedTransaction::InsufficientFunds);
                } else {
                    self.held -= amount;
                    self.total -= amount;
                    self.locked = true;
                }
                Ok(transaction.clone())
            }
        }
    }

    pub fn available_balance(&self) -> Amount {
        self.available
    }

    pub fn held_balance(&self) -> Amount {
        self.held
    }

    pub fn total_balance(&self) -> Amount {
        self.total
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
}
