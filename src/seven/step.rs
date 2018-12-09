use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Step<'a> {
    representation: char,
    dependencies: RefCell<Vec<&'a Step<'a>>>,
    done: bool,
}

impl<'a> Step<'a> {
    pub fn new(representation: char) -> Step<'a> {
        Step {
            representation,
            dependencies: RefCell::new(Vec::new()),
            done: false,
        }
    }

    pub fn get_representation(&self) -> char {
        self.representation
    }

    pub fn add_dependency(&self, dependency: &'a Step<'a>) {
        self.dependencies.borrow_mut().push(dependency);
    }

    pub fn get_time_to_finish(&self, base_time: u32) -> u32 {
        let ascii_code = self.representation as i32 as u32;

        // So A, which is 65, becomes 1, B becomes 2 etc.
        let normalized_ascii_code = ascii_code - 64;
        normalized_ascii_code + base_time
    }

    pub fn can_be_finished(&self) -> bool {
        self.dependencies.borrow().is_empty()
            || self.dependencies.borrow().iter().all(|d| d.is_finished())
    }

    pub fn finish(&mut self) {
        self.done = true
    }

    pub fn is_finished(&self) -> bool {
        self.done
    }
}
