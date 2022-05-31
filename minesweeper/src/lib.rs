#![feature(slice_flatten)]

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
pub struct Minesweeper<'a> {
    game: game::Game<'a>,
}

static RANDOM_CHOOSER: RandRandomChooser = RandRandomChooser {};

impl<'a> Minesweeper<'a> {
    /// Create new game
    pub fn new(height: u16, width: u16, bombs_amount: usize) -> Self {
        let game = game::Game::new(height, width, bombs_amount, &RANDOM_CHOOSER);
        Self { game }
    }

    /// View the game field
    pub fn view(&self) -> String {
        view::TerminalViewer::view_only_opened(&self.game.field)
    }

    /// Open cell and get "None" if game has not ended or "Some" if it has
    /// "Some" contains the game result
    pub fn open_cell(
        &mut self,
        coordinates: &cell::Coordinates,
    ) -> errors::MinesweeperResult<Option<&game::GameResult>> {
        let result = self.game.open_cell(coordinates)?;
        if !result {
            return Ok(None);
        }
        Ok(Some(self.game.get_result()))
    }
}
