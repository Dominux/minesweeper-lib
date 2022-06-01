use crate::{
    cell::{Cell, CellType, Coordinates},
    errors::{MinesweeperError, MinesweeperResult},
};

/// Main structure to play on
#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) cells: Vec<Cell>,
    pub(crate) height: u16,
}

impl Field {
    pub(crate) fn new(height: u16, width: u16) -> Self {
        let size = height * width;
        let cells = (0..size).map(|_| Cell::default()).collect();
        Self { cells, height }
    }

    /// Open the cell and return whether it contains a bomb or not
    pub(crate) fn open_cell(&mut self, coordinates: &Coordinates) -> MinesweeperResult<bool> {
        let cell = self.get_cell_by_coordinates(coordinates)?;
        cell.open();
        Ok(cell.is_bomb())
    }

    fn get_width(&self) -> u16 {
        self.cells.len() as u16 / self.height
    }

    fn get_cell_index_from_coordinates(
        &self,
        coordinates: &Coordinates,
    ) -> MinesweeperResult<usize> {
        if coordinates.column > self.get_width() {
            return Err(MinesweeperError::WrongCoordinates {
                column: coordinates.column,
                row: coordinates.row,
            });
        }

        Ok((coordinates.row * self.height + coordinates.column - 11) as usize)
    }

    pub(crate) fn get_cell_coordinates_from_index(&self, index: u16) -> Coordinates {
        let width = self.get_width();
        Coordinates {
            row: index / width + 1,
            column: index % width + 1,
        }
    }

    pub(crate) fn get_cell_by_coordinates(
        &mut self,
        coordinates: &Coordinates,
    ) -> MinesweeperResult<&mut Cell> {
        let i = self.get_cell_index_from_coordinates(coordinates)?;
        self.cells
            .get_mut(i)
            .ok_or(MinesweeperError::WrongCoordinates {
                column: coordinates.column,
                row: coordinates.row,
            })
    }

    pub(crate) fn set_celltype_by_index(&mut self, index: usize, _type: CellType) {
        self.cells[index].set_type(_type)
    }

    pub(crate) fn get_closed_cells_indexes(&self) -> Vec<usize> {
        (0..(self.cells.len() - 1))
            .filter(|i| !self.cells[*i].is_opened())
            .collect()
    }

    pub(crate) fn get_cells_neighbors(
        &mut self,
        coordinates: &Coordinates,
    ) -> [[Option<Coordinates>; 3]; 3] {
        let is_with_top = coordinates.row != 1;
        let is_with_left = coordinates.column != 1;
        let is_with_right = coordinates.column != (self.cells.len() as u16) / self.height;
        let is_with_bottom = coordinates.row != self.height;

        // Allocating desired result with nones
        let mut result = [[None, None, None], [None, None, None], [None, None, None]];

        // Filling it:

        // Top left
        if is_with_top && is_with_left {
            result[0][0] = Some(Coordinates {
                column: coordinates.column - 1,
                row: coordinates.row - 1,
            })
        }
        // Top middle
        if is_with_top {
            result[0][1] = Some(Coordinates {
                column: coordinates.column,
                row: coordinates.row - 1,
            })
        }
        // Top right
        if is_with_top && is_with_right {
            result[0][2] = Some(Coordinates {
                column: coordinates.column + 1,
                row: coordinates.row - 1,
            })
        }
        // Middle left
        if is_with_left {
            result[1][0] = Some(Coordinates {
                column: coordinates.column - 1,
                row: coordinates.row,
            });
        }
        // Middle middle
        result[1][1] = Some(coordinates.clone());
        // Middle right
        if is_with_right {
            result[1][2] = Some(Coordinates {
                column: coordinates.column + 1,
                row: coordinates.row,
            });
        }
        // Bottom left
        if is_with_left && is_with_bottom {
            result[2][0] = Some(Coordinates {
                column: coordinates.column - 1,
                row: coordinates.row + 1,
            });
        }
        // Bottom middle
        if is_with_bottom {
            result[2][1] = Some(Coordinates {
                column: coordinates.column,
                row: coordinates.row + 1,
            });
        }
        // Bottom right
        if is_with_right && is_with_bottom {
            result[2][2] = Some(Coordinates {
                column: coordinates.column + 1,
                row: coordinates.row + 1,
            });
        }

        result
    }

    /// Recursive cascadian openning cells
    pub(crate) fn cascadian_open(
        &mut self,
        coordinates: &Coordinates,
        open_anyway: bool,
    ) -> MinesweeperResult<bool> {
        let cell = self.get_cell_by_coordinates(coordinates)?;

        // 0. Not open cell if it is already opened
        if !open_anyway && cell.is_opened() {
            return Ok(false);
        }

        // 1. Openning cell
        cell.open();
        if cell.is_bomb() {
            return Ok(true);
        }

        // 2. If the cell has a bomb as a neighbor - pass
        if let CellType::Empty(n) = cell._type {
            if n != 0 {
                return Ok(false);
            }
        }

        // 3. Getting it's neighbors
        let neighbors = self.get_cells_neighbors(coordinates);

        for neighbor in neighbors.flatten() {
            match neighbor {
                None => continue,
                Some(c) => self.cascadian_open(c, false)?,
            };
        }

        Ok(false)
    }
}
