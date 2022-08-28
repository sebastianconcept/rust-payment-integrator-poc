
pub struct App {
  rules: Vec<String>
}

impl App {
  pub fn new() -> Self {
    Self { rules: Vec::new() }
  }
}