use crate::{
    cell::{CellType, Coordinates},
    field::Field,
    random_chooser::RandomChooser,
};

pub struct Game {
    field: Field,
    is_started: bool,
    pub bombs_amount: usize,
    pub random_chooser: &'static dyn RandomChooser,
}

impl Game {
    pub fn new(
        height: u16,
        width: u16,
        bombs_amount: usize,
        random_chooser: &'static dyn RandomChooser,
    ) -> Self {
        Self {
            field: Field::new(height, width),
            is_started: false,
            bombs_amount,
            random_chooser,
        }
    }

    /// Open the cell and return whether it contains a bomb or not
    pub fn open_cell(&mut self, coordinates: Coordinates) -> bool {
        let result = self.field.open_cell(coordinates);

        // If not started yet, then starting it and doing all the needed stuff
        if !self.is_started {
            self.on_start();
            self.is_started = true
        }

        result
    }

    /// Fill the field with bombs and according numbers in neighbors cells
    fn on_start(&mut self) {
        self.fill_with_bombs();
    }

    fn fill_with_bombs(&mut self) {
        let choosen_cells_indexes = {
            let closed_cells_indexes = self.field.get_closed_cells_indexes();

            self.random_chooser
                .choose_multiple(closed_cells_indexes, self.bombs_amount)
        };

        for i in choosen_cells_indexes {
            // Filling choosen cell with a bomb
            self.field.set_celltype_by_index(i, CellType::Bomb);

            // Incrementing it's empty neighbors bombs counter
            let neighbors_coordinates = self
                .field
                .get_cells_neighbors(self.field.get_cell_coordinates_from_index(i as u16))
                .into_iter()
                .flatten()
                .filter_map(|c| c);

            for neighbor in neighbors_coordinates {
                let cell = self.field.get_cell_by_coordinates(neighbor);

                if !cell.is_bomb() {
                    cell.increment_bombs_counter()
                }
            }
        }
    }
}
