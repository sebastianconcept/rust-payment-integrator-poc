use integrator::{app::App, cli::get_input_filename, csv::get_transactions_iter};

fn main() {
    let mut app = App::new();
    let input_filename = get_input_filename();
    let mut reader = get_transactions_iter(input_filename);
    for record in reader.records() {
        match record {
            Ok(r) => {
                app.process_record(r);
            }
            Err(_) => {}
        };
    }
}
