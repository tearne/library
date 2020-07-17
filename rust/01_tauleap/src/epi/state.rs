#[derive(serde::Serialize, Copy, Clone, Debug, PartialEq)]
pub struct State {
    pub s: u64,
    pub e: u64,
    pub i: u64,
    pub r: u64,
}

impl State {
    pub fn start() -> Self {
        State { s: 999, e: 0, i: 1, r: 0 }
    }

    pub fn size(&self) -> u64 {
        self.s + self.e + self.i + self.r
    }

    pub fn s(&self) -> f64 {
        self.s as f64
    }

    pub fn e(&self) -> f64 {
        self.e as f64
    }

    pub fn i(&self) -> f64 {
        self.i as f64
    }

    pub fn r(&self) -> f64 {
        self.r as f64
    }
}
