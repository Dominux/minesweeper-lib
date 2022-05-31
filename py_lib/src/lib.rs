use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

use minesweeper_lib;

#[pyclass(unsendable)]
struct Minesweeper {
    game: minesweeper_lib::Minesweeper,
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

    fn open_cell(
        &mut self,
        column: u16,
        row: u16,
    ) -> PyResult<Option<&minesweeper_lib::GameResult>> {
        let result = self.game.open_cell(column, row);
        if result.is_ok() {
            return Ok(result.unwrap());
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
fn minesweeper(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Minesweeper>()?;
    Ok(())
}
