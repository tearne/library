#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Environment {
    pub size: u64,
    pub infectious: u64,
}

impl Environment {
    pub fn size(&self) -> f64 {
        self.size as f64
    }

    pub fn infectious(&self) -> f64 {
        self.infectious as f64
    }
}
