#![feature(slice_flatten)]

pub use crate::errors::MinesweeperError;
pub use crate::game::GameResult;
use crate::random_chooser::RandRandomChooser;

mod cell;
mod errors;
mod field;
mod game;
mod random_chooser;
mod view;

#[cfg(test)]
mod tests;

/// The game itself
pub struct Minesweeper {
    game: game::Game,
}

impl Minesweeper {
    /// Create new game
    pub fn new(height: u16, width: u16, bombs_amount: usize) -> Self {
        let random_chooser = RandRandomChooser {};
        let game = game::Game::new(height, width, bombs_amount, Box::new(random_chooser));
        Self { game }
    }

    /// View the game field
    pub fn view(&self) -> String {
        view::TerminalViewer::view_only_opened(&self.game.field)
    }

    /// Open cell and get "None" if game has not ended or "Some" if it has
    /// "Some" contained the game result
    pub fn open_cell(
        &mut self,
        column: u16,
        row: u16,
    ) -> errors::MinesweeperResult<Option<&GameResult>> {
        let coordinates = cell::Coordinates { column, row };

        let result = self.game.open_cell(&coordinates)?;
        if !result {
            return Ok(None);
        }
        Ok(Some(self.game.get_result()))
    }
}
