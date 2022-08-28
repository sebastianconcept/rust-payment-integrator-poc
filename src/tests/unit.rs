use crate::{app::App, cli::get_command, csv::get_transactions_iter};

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
        },
        Err(_) => {},
    }
  }
  assert_eq!(records.len(), 5);
  assert_eq!(records[0].get(0).unwrap(), "deposit".to_string());
}

#[test]
fn can_parse_a_deposit_command() {
    let app = App::new();
    assert!(false);
}
