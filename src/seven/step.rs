#[derive(Debug)]
pub struct Step {
    representation: char,
    dependencies: Vec<char>,
    done: bool,
}

impl Step {
    pub fn new(representation: char) -> Step {
        Step {
            representation,
            dependencies: Vec::new(),
            done: false,
        }
    }

    pub fn get_representation(&self) -> char {
        self.representation
    }

    pub fn push_dependency(&mut self, dependency: char) {
        self.dependencies.push(dependency);
    }

    pub fn get_time_to_finish(&self, base_time: u32) -> u32 {
        let ascii_code = self.representation as i32 as u32;

        // So A, which is 65, becomes 1, B becomes 2 etc.
        let normalized_ascii_code = ascii_code - 64;
        normalized_ascii_code + base_time
    }

    pub fn can_be_finished(&self, finished_dependencies: &[char]) -> bool {
        self.dependencies.is_empty()
            || self
                .dependencies
                .iter()
                .all(|d| finished_dependencies.contains(d))
    }

    fn is_finished(&self) -> bool {
        self.done
    }

    fn finish(&mut self) {
        self.done = true;
    }
}
