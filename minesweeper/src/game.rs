use crate::field::Field;

pub struct Game {
    field: Field,
    is_started: bool,
}

impl Game {
    pub fn new(height: u16, width: u16) -> Self {
        Self {
            field: Field::new(height, width),
            is_started: false,
        }
    }

    pub fn open(&mut self) {
        if !self.is_started {
            // If not started yet, then start the game and fill the field with bombs
            // and according numbers in neighbors cells

            self.is_started = true;
            return;
        }
    }
}
