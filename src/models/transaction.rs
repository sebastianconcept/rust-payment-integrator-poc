use csv::StringRecord;
use serde::Deserialize;

type Result<T> = std::result::Result<T, InvalidTransactionType>;
#[derive(Debug, Clone)]
pub struct InvalidTransactionType;

// type, client, tx, amount
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Transaction {
    pub kind: TransactionType,
    pub client_id: u16,
    pub id: u32,
    pub amount: f32,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
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
            None => Err(InvalidTransactionType),
            Some(value) => {
                let kind;
                match value {
                    "deposit" => kind = TransactionType::Deposit,
                    "withdrawal" => kind = TransactionType::Withdrawal,
                    "dispute" => kind = TransactionType::Dispute,
                    "resolve" => kind = TransactionType::Resolve,
                    "chargeback" => kind = TransactionType::Chargeback,
                    _ => return Err(InvalidTransactionType),
                };
                Ok(Self {
                    kind: kind,
                    client_id: record.get(1).unwrap().trim().parse::<u16>().unwrap(),
                    id: record.get(2).unwrap().trim().parse::<u32>().unwrap(),
                    amount: record.get(3).unwrap().trim().parse::<f32>().unwrap(),
                })
            }
        }
    }
}
