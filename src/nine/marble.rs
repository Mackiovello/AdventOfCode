use std::cell::Cell;

pub struct Marbles {
    items: Vec<Marble>,
    current: Cell<usize>,
}

impl Marbles {
    pub fn new(number_of_marbles: u32) -> Marbles {
        Marbles {
            items: (0..number_of_marbles).map(Marble::new).collect(),
            current: Cell::new(0),
        }
    }

    pub fn get_next(&self) -> &Marble {
        let marble = &self.items.get(self.current.get()).unwrap_or_else(|| {
            self.current.set(0);
            &self.items[self.current.get()]
        });
        self.current.update(|v| v + 1);
        marble
    }
}

pub struct Marble {
    value: u32,
}

impl Marble {
    fn new(value: u32) -> Marble {
        Marble { value }
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}
