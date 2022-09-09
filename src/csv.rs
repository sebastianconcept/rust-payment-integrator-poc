use std::fs::File;

use csv::{Reader, ReaderBuilder, Trim};

pub fn get_transactions_iter(filename: String) -> Reader<File> {
    let path = filename;
    let msg = format!("Couldn't read from {}", path);
    ReaderBuilder::new()
        .has_headers(false)
        .trim(Trim::All)
        .delimiter(b',')
        .from_path(path)
        .expect(&msg)
}