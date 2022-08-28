use std::fs::File;

use csv::{Reader};

pub fn get_transactions_iter(filename: String) -> Reader<File>{
  let path = filename;
  let msg = format!("Couldn't read from {}", path);
  let reader = Reader::from_path(path)
    .expect(&msg);
  reader
}