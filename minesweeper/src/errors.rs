use thiserror::Error;

#[derive(Debug, Error)]
pub enum MinesweeperError {
    #[error("You cannot open cells after the end of the game")]
    OpeningCellsNotAllowedAfterGameEnd,
    #[error("Coordinates out of range (column: {column:?}, row: {row:?})")]
    WrongCoordinates { column: u16, row: u16 },
}

pub type MinesweeperResult<T> = Result<T, MinesweeperError>;
