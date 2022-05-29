/// Enum type for cells
///
/// It shows whether a cell contains a bomb or not.
/// Aditionally, in case it's empty, it has info about
/// how much neighbors cells contain bombs
pub(crate) enum CellType {
    Bomb,
    Empty(u16),
}

pub(crate) struct Cell {
    _type: CellType,
    _is_opened: bool,
}

impl Default for Cell {
    #[inline]
    fn default() -> Self {
        Self {
            _type: CellType::Empty(0),
            _is_opened: false,
        }
    }
}

impl Cell {
    pub fn open(&mut self) {
        self._is_opened = true
    }

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub fn is_bomb(&self) -> bool {
        matches!(self._type, CellType::Bomb)
    }

    pub(crate) fn set_type(&mut self, _type: CellType) {
        self._type = _type
    }

    /// Increment the cell's bombs counter
    ///
    /// Panics if it contains a bomb
    pub(crate) fn increment_bombs_counter(&mut self) {
        match self._type {
            CellType::Empty(c) => self.set_type(CellType::Empty(c + 1)),
            CellType::Bomb => panic!("the cell contains a bomb"),
        }
    }
}

#[derive(Debug)]
pub struct Coordinates {
    pub row: u16,
    pub column: u16,
}

impl Clone for Coordinates {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            column: self.column,
        }
    }
}
