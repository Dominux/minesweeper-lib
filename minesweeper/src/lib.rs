#![feature(slice_flatten)]

pub use crate::cell::CellType;
pub use crate::errors::MinesweeperError;
pub use crate::game::GameResult;
pub use crate::view::SimpleViewCell;

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
#[derive(Clone, PartialEq)]
pub struct Minesweeper {
    game: game::Game<RandRandomChooser>,
}

impl Minesweeper {
    /// Create new game
    pub fn new(height: u16, width: u16, bombs_amount: usize) -> Self {
        let random_chooser = RandRandomChooser {};
        let game = game::Game::new(height, width, bombs_amount, random_chooser);
        Self { game }
    }

    /// View the game field
    pub fn view(&self, show_all_bombs_after_explosion: bool) -> Vec<Vec<SimpleViewCell>> {
        view::SimpleView::view(
            &self.game.field,
            self.game.is_ended() && show_all_bombs_after_explosion,
        )
    }

    /// View the game field as string
    pub fn as_string(&self) -> String {
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
