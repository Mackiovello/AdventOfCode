use std::cell::Cell;

use super::marble::Marble;

pub struct Circle {
    current: Cell<usize>,
    marbles: Vec<u32>,
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            current: Cell::new(0),
            marbles: Vec::new(),
        }
    }
}
