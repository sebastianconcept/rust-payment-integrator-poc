extern crate clap;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_input_filename() -> String {
    let args = get_arguments();
    let found = args
        .get_one::<String>("input_filename")
        .expect("Failed to get the CSV filename to use as input");
    found.to_string()
}

pub fn get_command<'a>() -> Command<'a> {
    Command::new("integrator")
    .version("1.0")
    .author("Sebastian Sastre <sebastianconcept@gmail.com>")
    .about("PoC payment system to demonstrate transactions processing and account maintenance using CSV files.")
    .arg(
        Arg::new("input_filename")
            .multiple(false)
            .action(ArgAction::Append)
            .value_parser(value_parser!(String))
            .help("Defines the CSV filename to use as input.")
            .required(true)
            .value_name("FILENAME")
            .takes_value(true),
    )
}

fn get_arguments<'a>() -> ArgMatches {
    get_command().get_matches()
}
