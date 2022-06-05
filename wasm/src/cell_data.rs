use minesweeper_lib::SimpleViewCell;

#[derive(PartialEq)]
pub struct CellData {
    pub _type: Option<minesweeper_lib::CellType>,
    pub is_opened: bool,
}

impl CellData {
    pub fn new(cell: &SimpleViewCell) -> Self {
        match cell._type {
            Some(t) => Self {
                _type: Some(t.clone()),
                is_opened: cell.is_opened,
            },
            None => Self {
                _type: None,
                is_opened: cell.is_opened,
            },
        }
    }
}
