use crate::models::account::{RejectedTransaction, Result};
use csv::StringRecord;

#[derive(Debug, Clone)]
pub struct InvalidTransactionType;

pub type ClientID = u16;
pub type TransactionID = u32;
pub type Amount = f32;

// type, client, tx, amount
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub kind: TransactionType,
    pub client_id: ClientID,
    pub id: TransactionID,
    pub amount: Option<Amount>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl Transaction {
    pub fn from_record(record: StringRecord) -> Result<Self> {
        match record.get(0) {
            None => Err(RejectedTransaction::InvalidType),
            Some(value) => match value {
                "deposit" => Self::new_deposit(record),
                "withdrawal" => Self::new_withdrawal(record),
                "dispute" => Self::new_dispute(record),
                "resolve" => Self::new_resolve(record),
                "chargeback" => Self::new_chargeback(record),
                _ => return Err(RejectedTransaction::InvalidInput),
            },
        }
    }

    pub fn basic_new(
        record: StringRecord,
        kind: TransactionType,
        amount: Option<Amount>,
    ) -> Result<Self> {
        Ok(Self {
            kind,
            client_id: record.get(1).unwrap().trim().parse::<ClientID>().unwrap(),
            id: record
                .get(2)
                .unwrap()
                .trim()
                .parse::<TransactionID>()
                .unwrap(),
            amount,
        })
    }

    pub fn new_deposit(record: StringRecord) -> Result<Self> {
        let amount;
        match record.get(3) {
            None => return Err(RejectedTransaction::InvalidInput),
            Some(value) => {
                let a = value.trim().parse::<Amount>();
                match a {
                    Err(err) => return Err(RejectedTransaction::InvalidInput),
                    Ok(value) => amount = Some(value),
                }
            }
        };
        Self::basic_new(record, TransactionType::Deposit, amount)
    }

    pub fn new_withdrawal(record: StringRecord) -> Result<Self> {
        let amount;
        match record.get(3) {
            None => return Err(RejectedTransaction::InvalidInput),
            Some(value) => {
                let a = value.trim().parse::<Amount>();
                match a {
                    Err(err) => return Err(RejectedTransaction::InvalidInput),
                    Ok(value) => amount = Some(value),
                }
            }
        };
        Self::basic_new(record, TransactionType::Withdrawal, amount)
    }

    pub fn new_dispute(record: StringRecord) -> Result<Self> {
        Self::basic_new(record, TransactionType::Dispute, None)
    }

    pub fn new_resolve(record: StringRecord) -> Result<Self> {
        Self::basic_new(record, TransactionType::Resolve, None)
    }

    pub fn new_chargeback(record: StringRecord) -> Result<Self> {
        Self::basic_new(record, TransactionType::Chargeback, None)
    }
}
