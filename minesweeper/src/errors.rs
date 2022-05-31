use thiserror::Error;

#[derive(Debug, Error)]
pub enum MinesweeperError {
    #[error("You cannot open cells after the end of the game")]
    OpeningCellsNotAllowedAfterGameEnd,
}

pub type MinesweeperResult<T> = Result<T, MinesweeperError>;
