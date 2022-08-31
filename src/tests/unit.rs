use csv::StringRecord;

use crate::{
    app::App,
    cli::get_command,
    csv::get_transactions_iter,
    models::{
        transaction::{Transaction, TransactionType},
        transactions::{transactions_size, TRANSACTIONS},
    },
};

fn transactions_reset() {
    TRANSACTIONS
        .write()
        .expect("Cannot write transactions")
        .reset();
}

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
            assert_eq!(tx.amount.unwrap(), 1.0f32);
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
            assert_eq!(tx.amount.unwrap(), 3.0f32);
        }
    }
}

#[test]
fn deposit_can_increase_account_balance() {
    transactions_reset();
    let mut app = App::new();
    let record = StringRecord::from(vec!["deposit", "    2", "5      ", " 3.0 "]);
    let tx = Transaction::from_record(record);
    match tx {
        Err(_err) => assert!(false),
        Ok(tx) => {
            let client_id = tx.client_id.clone();
            let before = app.get_available_balance(client_id);
            assert_eq!(before, 0f32);
            app.process(tx);
            let after = app.get_available_balance(client_id);
            assert_ne!(before, after);
            assert_eq!(after, 3.0f32);
        }
    }
    let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "15", "1.5"]));
    let client_id = tx2.as_ref().unwrap().client_id;
    app.process(tx2.unwrap());
    let after2 = app.get_available_balance(client_id);
    assert_eq!(after2, 4.5f32);
    assert_eq!(app.get_held_balance(client_id) + app.get_available_balance(client_id), app.get_total_balance(client_id));
    assert_eq!(app.is_locked(client_id), false);
}

#[test]
fn withdrawal_can_decrease_account_balance() {
    transactions_reset();
    let mut app = App::new();
    let record = StringRecord::from(vec!["deposit", "    2", "5      ", " 3.0 "]);
    let tx = Transaction::from_record(record);
    match tx {
        Err(_err) => assert!(false),
        Ok(tx) => {
            let client_id = tx.client_id.clone();
            app.process(tx);
            let after = app.get_available_balance(client_id);
            assert_eq!(after, 3.0f32);
        }
    }
    let tx2 = Transaction::from_record(StringRecord::from(vec!["withdrawal", "2", "15", "1.3"]));
    let client_id = tx2.as_ref().unwrap().client_id;
    app.process(tx2.unwrap());
    let after2 = app.get_available_balance(client_id);
    assert_eq!(after2, 3.0 - 1.3f32);
    let size = transactions_size();
    assert_eq!(size, 2);
    assert_eq!(app.get_held_balance(client_id) + app.get_available_balance(client_id), app.get_total_balance(client_id));
    assert_eq!(app.is_locked(client_id), false);
}

#[test]
fn dispute_increase_disputed_balance_and_maintain_total() {
    transactions_reset();
    let mut app = App::new();
    let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
    let client_id = tx1.as_ref().unwrap().client_id;
    app.process(tx1.unwrap());
    let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
    app.process(tx2.unwrap());
    let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "4", ""]));
    app.process(tx3.unwrap());
    let account = app.get_account(client_id);
    let total = account.total_balance();
    let available = account.available_balance();
    assert_eq!(available, total - 2.0f32);
    assert_eq!(app.get_held_balance(client_id) + app.get_available_balance(client_id), app.get_total_balance(client_id));
    assert_eq!(app.is_locked(client_id), false);
}

#[test]
fn resolve_decrease_held_balances_increase_available_and_maintain_total() {
    transactions_reset();
    let mut app = App::new();
    let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
    let client_id = tx1.as_ref().unwrap().client_id;
    app.process(tx1.unwrap());
    let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
    app.process(tx2.unwrap());
    let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "4", ""]));
    app.process(tx3.unwrap());
    let held_before = app.get_account(client_id).held_balance().clone();
    let total_before = app.get_account(client_id).total_balance().clone();
    assert_ne!(held_before, 0f32);
    assert_eq!(held_before, 2.0f32);
    assert_eq!(total_before, 3.5f32);
    let tx4 = Transaction::from_record(StringRecord::from(vec!["resolve", "2", "4", ""]));
    app.process(tx4.unwrap());
    let held_after = app.get_account(client_id).held_balance().clone();
    let total_after = app.get_account(client_id).total_balance().clone();
    assert_ne!(held_after, 2.0f32);
    assert_eq!(held_after, 0f32);
    assert_eq!(total_after, 3.5f32);
    assert_eq!(app.get_held_balance(client_id) + app.get_available_balance(client_id), app.get_total_balance(client_id));
    assert_eq!(app.is_locked(client_id), false);
}



#[test]
fn chargeback_decreases_held_and_total_balances_and_locks_account() {
    transactions_reset();
    let mut app = App::new();
    let tx1 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "4", "2.0 "]));
    let client_id = tx1.as_ref().unwrap().client_id;
    app.process(tx1.unwrap());
    let tx2 = Transaction::from_record(StringRecord::from(vec!["deposit", "2", "5", "1.5"]));
    app.process(tx2.unwrap());
    let tx3 = Transaction::from_record(StringRecord::from(vec!["dispute", "2", "4", ""]));
    app.process(tx3.unwrap());
    let held_before = app.get_account(client_id).held_balance().clone();
    let total_before = app.get_account(client_id).total_balance().clone();
    assert_ne!(held_before, 0f32);
    assert_eq!(held_before, 2.0f32);
    assert_eq!(total_before, 3.5f32);
    let tx4 = Transaction::from_record(StringRecord::from(vec!["chargeback", "2", "4", ""]));
    app.process(tx4.unwrap());
    let held_after = app.get_account(client_id).held_balance().clone();
    let total_after = app.get_account(client_id).total_balance().clone();
    assert!(app.get_account(client_id).is_locked());
    assert_ne!(held_after, 2f32);
    assert_eq!(held_after, 0f32);
    assert_eq!(total_after, 1.5f32);
    assert_eq!(app.get_held_balance(client_id) + app.get_available_balance(client_id), app.get_total_balance(client_id));
    assert_eq!(app.is_locked(client_id), true);
}