use crate::cell::Cell;

/// Main structure to play on
pub struct Field {
    pub(crate) cells: Vec<Cell>,
    pub(crate) height: u16,
}

impl Field {
    pub(crate) fn new(height: u16, width: u16) -> Self {
        let size = height * width;
        let cells = (0..size).map(|_| Cell::default()).collect();
        Self { cells, height }
    }
}
