use crate::{app::App, cli::get_command};

#[test]
fn can_parse_input_file_from_command_line() {
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
fn can_parse_a_deposit_command() {
    let app = App::new();
    assert!(false);
}
