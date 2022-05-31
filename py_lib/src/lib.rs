use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

use minesweeper_lib;

#[pyclass(unsendable)]
struct Minesweeper {
    game: minesweeper_lib::Minesweeper,
}

#[pyclass]
enum GameResult {
    Victory,
    Defeat,
}

impl From<&minesweeper_lib::GameResult> for GameResult {
    fn from(r: &minesweeper_lib::GameResult) -> Self {
        match r {
            minesweeper_lib::GameResult::Victory => GameResult::Victory,
            minesweeper_lib::GameResult::Defeat => GameResult::Defeat,
        }
    }
}

#[pymethods]
impl Minesweeper {
    #[new]
    fn new(height: u16, width: u16, bombs_amount: usize) -> Self {
        let game = minesweeper_lib::Minesweeper::new(height, width, bombs_amount);
        Self { game }
    }

    #[getter]
    fn view(&self) -> PyResult<String> {
        Ok(self.game.view())
    }

    fn open_cell(&mut self, column: u16, row: u16) -> PyResult<Option<GameResult>> {
        let result = self.game.open_cell(column, row);

        if result.is_ok() {
            let result = result.unwrap();
            if let None = result {
                return Ok(None);
            }

            return Ok(Some(result.unwrap().into()));
        }

        let error = result.unwrap_err();
        match error {
            minesweeper_lib::MinesweeperError::WrongCoordinates { column: _, row: _ } => {
                Err(PyValueError::new_err(error.to_string()))
            }
            minesweeper_lib::MinesweeperError::OpeningCellsNotAllowedAfterGameEnd => {
                Err(PyRuntimeError::new_err(error.to_string()))
            }
        }
    }
}

/// Minesweeper lib
#[pymodule]
fn minesweeper(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Minesweeper>()?;
    m.add_class::<GameResult>()?;
    Ok(())
}
