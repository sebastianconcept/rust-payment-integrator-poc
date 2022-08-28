use csv::StringRecord;

use crate::models::*;

pub struct App {
    rules: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn process(&self, transaction: StringRecord) {
        println!("Processing {:?}", transaction)
    }
}
