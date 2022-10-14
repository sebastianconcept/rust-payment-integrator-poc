extern crate lazy_static;
extern crate mut_static;

use integrator::{app::App, cli::get_input_filename, csv::get_transactions_iter};

fn main() {
    let mut app = App::new();
    let input_filename = get_input_filename();
    let mut reader = get_transactions_iter(input_filename);
    for record in reader.records() {
        match record {
            Ok(r) => match app.process_record(r) {
                Ok(tx) => {
                    let client = tx.client_id;
                    let account = app.get_account(client);
                    let available = format!("{:.4}", account.available_balance());
                    let held = format!("{:.4}", account.held_balance());
                    let total = format!("{:.4}", account.total_balance());
                    let locked = account.is_locked();
                    let message = format!("{},{},{},{},{}", client, available, held, total, locked);
                    app.output_write(message);
                }
                _ => {
                    // Silently ignore rejected transactions
                }
            },
            Err(_) => {}
        };
    }
}
