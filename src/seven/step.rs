use core::cell::{Cell, RefCell};

#[derive(Debug, Clone)]
pub struct Step {
    representation: char,
    dependencies: RefCell<Vec<usize>>,
    done: Cell<bool>,
}

impl Step {
    pub fn new(representation: char) -> Step {
        Step {
            representation,
            dependencies: RefCell::new(Vec::new()),
            done: Cell::new(false),
        }
    }

    pub fn get_representation(&self) -> char {
        self.representation
    }

    pub fn add_dependency(&self, dependency: usize) {
        self.dependencies.borrow_mut().push(dependency);
    }

    pub fn get_time_to_finish(&self, base_time: u32) -> u32 {
        let ascii_code = self.representation as i32 as u32;

        // So A, which is 65, becomes 1, B becomes 2 etc.
        let normalized_ascii_code = ascii_code - 64;
        normalized_ascii_code + base_time
    }

    pub fn can_be_finished(&self, steps: Vec<&Step>) -> bool {
        if self.dependencies.borrow().is_empty() {
            return true;
        }

        let indexes = self.dependencies.borrow();
        for i in indexes.iter() {
            if !steps[*i].is_finished() {
                return false;
            }
        }

        true
    }

    pub fn finish(&self) {
        self.done.set(true)
    }

    pub fn is_finished(&self) -> bool {
        self.done.get()
    }
}
