use csv::StringRecord;

use crate::{
    app::App,
    cli::{self, get_command},
    csv::get_transactions_iter,
    models::transaction::{Transaction, TransactionType},
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
    assert_eq!(records.len(), 5);
    assert_eq!(records[0].get(0).unwrap(), "deposit".to_string());
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
            assert_eq!(tx.amount, 1.0f32);
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
            assert_eq!(tx.amount, 3.0f32);
        }
    }
}

#[test]
fn deposit_can_increase_account_balance() {
    let mut app = App::new();
    let record = StringRecord::from(vec!["deposit", "    2", "5      ", " 3.0 "]);
    let tx = Transaction::from_record(record);
    match tx {
        Err(_err) => assert!(false),
        Ok(tx) => {
            let tid = tx.client_id.clone();
            let before = app.get_available_balance(tid);
            assert_eq!(before, 0f32);
            app.process(tx);
            let after = app.get_available_balance(tid);
            assert_ne!(before, after);
            assert_eq!(after, 3.0f32);
        }
    }
}

#[test]
fn dispute_increase_disputed_balance_and_maintain_total() {
    let mut app = App::new();
    let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
    let client_id = tx1.as_ref().unwrap().client_id;
    app.process(tx1.unwrap());
    let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
    app.process(tx2.unwrap());
    let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "6", "0.5 "]));
    app.process(tx3.unwrap());
    let account = app.get_account(client_id);
    let total = account.total_balance();
    let available = account.available_balance();
    assert_eq!(available, total - 0.5f32)
}
