#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use integrator::{
    app::{App, OUTPUT},
    cli::get_input_filename,
    csv::get_transactions_iter,
};

fn main() {
    let mut app = App::new();
    let input_filename = get_input_filename();
    let mut reader = get_transactions_iter(input_filename);
    for record in reader.records() {
        match record {
            Ok(r) => match app.process_record(r) {
                Ok((tx, account)) => {
                    let client = account.client_id;
                    let available = account.available_balance();
                    let held = account.held_balance();
                    let total = account.total_balance();
                    let locked = account.is_locked();
                    let message = format!("{},{},{},{},{}", client, available, held, total, locked);
                    OUTPUT
                        .write()
                        .expect("Failed to get output write access")
                        .write(message)
                }
                _RejectedTransaction => {}
            },
            Err(_) => {}
        };
    }
}
