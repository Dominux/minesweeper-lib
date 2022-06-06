use minesweeper_lib::SimpleViewCell;

#[derive(PartialEq, Clone, Copy)]
pub struct CellData {
    pub _type: Option<minesweeper_lib::CellType>,
    pub is_opened: bool,
    pub coordinates: Coordinates,
}

impl CellData {
    pub fn new(cell: &SimpleViewCell, cords: Coordinates) -> Self {
        match cell._type {
            Some(t) => Self {
                _type: Some(t.clone()),
                is_opened: cell.is_opened,
                coordinates: cords,
            },
            None => Self {
                _type: None,
                is_opened: cell.is_opened,
                coordinates: cords,
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub row: u16,
    pub column: u16,
}

impl Coordinates {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row: row as u16,
            column: column as u16,
        }
    }
}
