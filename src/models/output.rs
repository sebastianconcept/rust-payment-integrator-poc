pub struct Output {}

impl Output {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(&self, string: String) {
        println!("{}", string);
    }
}
