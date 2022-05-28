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
    pub fn open() {
        unimplemented!()
    }

    pub fn is_opened(&self) -> bool {
        self._is_opened
    }

    pub(crate) fn set_type(&mut self, _type: CellType) {
        self._type = _type
    }
}
