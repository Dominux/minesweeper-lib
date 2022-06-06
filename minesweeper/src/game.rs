use crate::{
    cell::{CellType, Coordinates},
    errors::{MinesweeperError, MinesweeperResult},
    field::Field,
    random_chooser::RandomChooser,
};

#[derive(Clone, PartialEq)]
pub struct Game<T>
where
    T: RandomChooser,
{
    pub(crate) field: Field,
    pub bombs_amount: usize,
    pub random_chooser: T,
    state: GameState,
}

impl<T> Game<T>
where
    T: RandomChooser,
{
    pub fn new(height: u16, width: u16, bombs_amount: usize, random_chooser: T) -> Self {
        Self {
            field: Field::new(height, width),
            bombs_amount,
            random_chooser,
            state: GameState::NotStarted,
        }
    }

    /// Open the cell and return whether the game has ended or not
    pub fn open_cell(&mut self, coordinates: &Coordinates) -> MinesweeperResult<bool> {
        if self.is_ended() {
            return Err(MinesweeperError::OpeningCellsNotAllowedAfterGameEnd);
        }

        // If not started yet, then starting it and doing all the needed stuff
        if !self.is_started() {
            self.field.open_cell(coordinates)?;
            self.start()?;
            self.field.cascadian_open(coordinates, true)?;
            return Ok(false);
        }

        let result = self.field.cascadian_open(coordinates, false)?;

        if result {
            self.end(GameResult::Defeat);
            return Ok(true);
        }

        // If all no-bomb cells are opened - the game is won
        if self
            .field
            .cells
            .iter()
            .filter(|c| !c.is_bomb())
            .all(|c| c.is_opened())
        {
            self.end(GameResult::Victory);
            return Ok(true);
        }

        Ok(false)
    }

    fn fill_with_bombs(&mut self) -> MinesweeperResult<()> {
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
                let cell = self.field.get_cell_by_coordinates(&neighbor)?;

                if !cell.is_bomb() {
                    cell.increment_bombs_counter()
                }
            }
        }

        Ok(())
    }

    //////////////////////////////////////////////////////////////
    //  State features
    //////////////////////////////////////////////////////////////

    pub fn is_started(&self) -> bool {
        !matches!(self.state, GameState::NotStarted)
    }

    fn start(&mut self) -> MinesweeperResult<()> {
        // Fill the field with bombs and according numbers in neighbors cells
        self.fill_with_bombs()?;
        self.state = GameState::Started;
        Ok(())
    }

    pub fn is_ended(&self) -> bool {
        matches!(self.state, GameState::Ended(_))
    }

    fn end(&mut self, result: GameResult) {
        self.state = GameState::Ended(result)
    }

    pub fn get_result(&self) -> &GameResult {
        match &self.state {
            GameState::Ended(r) => r,
            _ => panic!("Game isn't ended yet"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    Victory,
    Defeat,
}

#[derive(Clone, PartialEq)]
enum GameState {
    NotStarted,
    Started,
    Ended(GameResult),
}
