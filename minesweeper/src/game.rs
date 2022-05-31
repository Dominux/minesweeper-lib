use crate::{
    cell::{CellType, Coordinates},
    field::Field,
    random_chooser::RandomChooser,
};

pub struct Game<'a> {
    pub(crate) field: Field,
    pub bombs_amount: usize,
    pub random_chooser: &'a dyn RandomChooser,
    state: GameState,
}

impl<'a> Game<'a> {
    pub fn new(
        height: u16,
        width: u16,
        bombs_amount: usize,
        random_chooser: &'a dyn RandomChooser,
    ) -> Self {
        Self {
            field: Field::new(height, width),
            bombs_amount,
            random_chooser,
            state: GameState::NotStarted,
        }
    }

    /// Open the cell and return whether it contains a bomb or not
    pub fn open_cell(&mut self, coordinates: &Coordinates) -> bool {
        let result = self.cascadian_open(coordinates);

        if result {
            self.end(GameResult::Defeat);
            return result;
        }

        // If not started yet, then starting it and doing all the needed stuff
        if !self.is_started() {
            self.start();
        }

        false
    }

    /// Recursive cascadian openning cells
    fn cascadian_open(&mut self, coordinates: &Coordinates) -> bool {
        // 0. Not open cell if it is already opened
        if self.field.is_cell_opened(coordinates) {
            return false;
        }

        // 1. Openning cell
        let result = self.field.open_cell(coordinates);

        if result {
            return result;
        }

        // 2. Getting it's neighbors
        let neighbors = self.field.get_cells_neighbors(coordinates);

        for neighbor in neighbors.flatten() {
            match neighbor {
                None => continue,
                Some(c) => self.cascadian_open(c),
            };
        }

        false
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
                .get_cells_neighbors(&self.field.get_cell_coordinates_from_index(i as u16))
                .into_iter()
                .flatten()
                .filter_map(|c| c);

            for neighbor in neighbors_coordinates {
                let cell = self.field.get_cell_by_coordinates(&neighbor);

                if !cell.is_bomb() {
                    cell.increment_bombs_counter()
                }
            }
        }
    }

    //////////////////////////////////////////////////////////////
    //  State features
    //////////////////////////////////////////////////////////////

    pub fn is_started(&self) -> bool {
        !matches!(self.state, GameState::NotStarted)
    }

    fn start(&mut self) {
        // Fill the field with bombs and according numbers in neighbors cells
        self.fill_with_bombs();
        self.state = GameState::Started
    }

    pub fn is_ended(&self) -> bool {
        matches!(self.state, GameState::Ended(_))
    }

    fn end(&mut self, result: GameResult) {
        self.state = GameState::Ended(result)
    }

    fn get_result(&self) -> &GameResult {
        match &self.state {
            GameState::Ended(r) => r,
            _ => panic!("Game isn't ended yet"),
        }
    }
}

pub enum GameResult {
    Victory,
    Defeat,
}

enum GameState {
    NotStarted,
    Started,
    Ended(GameResult),
}
