use csv::Reader;
use integrator::{app::App, cli::get_input_filename};

fn main() {
    let input_filename = get_input_filename();
    let app = App::new();
}
