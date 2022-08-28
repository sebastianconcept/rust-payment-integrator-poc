use integrator::{app::App, cli::get_input_filename, csv::get_transactions_iter};

fn main() {
    let app = App::new();
    let input_filename = get_input_filename();
    let mut transactions = get_transactions_iter(input_filename);
    for transaction in transactions.records() {
        match transaction {
            Ok(tx) => {
                app.process(tx);
            }
            Err(_) => {}
        };
    }
}
