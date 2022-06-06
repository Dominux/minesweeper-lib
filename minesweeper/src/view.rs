use crate::{
    cell::{Cell, CellType},
    field::Field,
};

pub(crate) struct SimpleView;

pub struct SimpleViewCell<'a> {
    pub _type: Option<&'a CellType>,
    pub is_opened: bool,
}

impl<'a> SimpleViewCell<'a> {
    fn new(cell: &'a Cell, show_bomb: bool) -> Self {
        if cell.is_opened() {
            Self {
                _type: Some(&cell._type),
                is_opened: true,
            }
        } else {
            let _type = if show_bomb && matches!(cell._type, CellType::Bomb) {
                Some(&CellType::Bomb)
            } else {
                None
            };
            Self {
                _type,
                is_opened: false,
            }
        }
    }
}

impl SimpleView {
    pub(crate) fn view(field: &Field, show_bombs: bool) -> Vec<Vec<SimpleViewCell>> {
        field
            .cells
            .chunks(field.get_width() as usize)
            .map(|row| {
                row.iter()
                    .map(|c| SimpleViewCell::new(c, show_bombs))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}
pub struct TerminalViewer;

impl TerminalViewer {
    #[allow(dead_code)]
    pub(crate) fn view(field: &Field) -> String {
        field
            .cells
            .chunks(field.get_width() as usize)
            .map(|row| {
                row.iter()
                    .map(|c| match c._type {
                        CellType::Bomb => "*".to_string(),
                        CellType::Empty(b) => b.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn view_only_opened(field: &Field) -> String {
        field
            .cells
            .chunks(field.get_width() as usize)
            .map(|row| {
                row.iter()
                    .map(|c| {
                        if c.is_opened() {
                            match c._type {
                                CellType::Bomb => "*".to_string(),
                                CellType::Empty(b) => b.to_string(),
                            }
                        } else {
                            "█".to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
