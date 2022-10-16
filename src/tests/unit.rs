use csv::StringRecord;
use fraction::Decimal;

use crate::{
    cli::get_command,
    csv::get_transactions_iter,
    models::{
        transaction::{Transaction, TransactionType},
    },
};

#[test]
fn can_parse_input_filename_from_command_line() {
    let matches = get_command()
        .try_get_matches_from(["integrator", "input_file.csv"])
        .unwrap();
    assert!(matches.contains_id("input_filename"));
    assert_eq!(
        matches.value_of("input_filename").unwrap(),
        "input_file.csv"
    );
}

#[test]
fn can_read_a_record_streamed_from_a_csv_input_file() {
    let mut transactions_iter = get_transactions_iter("input/scenario1.csv".to_string());
    let mut records = Vec::new();
    for record in transactions_iter.records() {
        match record {
            Ok(record) => {
                records.push(record);
            }
            Err(_) => {}
        }
    }
    assert_eq!(records.len(), 6);
    assert_eq!(records[0].get(0).unwrap(), "type".to_string());
    assert_eq!(records[0].get(1).unwrap(), "client".to_string());
    assert_eq!(records[1].get(0).unwrap(), "deposit".to_string());
}

#[test]
fn can_parse_a_deposit_command() {
    let record = StringRecord::from(vec!["deposit", "    1", "      1", " 1.0 "]);
    let tx = Transaction::from_record(record);
    match tx {
        Err(_err) => assert!(false),
        Ok(tx) => {
            let kind = tx.kind;
            assert_eq!(kind, TransactionType::Deposit);
            assert_eq!(tx.client_id, 1u16);
            assert_eq!(tx.id, 1u32);
            assert_eq!(tx.amount.unwrap(), Decimal::from(1.0));
        }
    }
}

#[test]
fn can_parse_a_withdrawal_command() {
    let record = StringRecord::from(vec!["withdrawal", "    2", "5      ", " 3.0 "]);
    let tx = Transaction::from_record(record);
    match tx {
        Err(_err) => assert!(false),
        Ok(tx) => {
            let kind = tx.kind;
            assert_eq!(kind, TransactionType::Withdrawal);
            assert_eq!(tx.client_id, 2u16);
            assert_eq!(tx.id, 5u32);
            assert_eq!(tx.amount.unwrap(), Decimal::from(3.0));
        }
    }
}

