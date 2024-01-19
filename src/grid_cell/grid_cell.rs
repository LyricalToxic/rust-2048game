use std::iter::Enumerate;
use std::slice::Iter;

use crate::game_cell::Cell2048;

pub struct GridCell2048 {
    size: u32,
    cells: Vec<Vec<Cell2048>>,
}

impl GridCell2048 {
    pub fn new(size: u32) -> Self {
        let cells = (0..size).map(|row| {
            (0..size).map(|col| {
                Cell2048::new("", row, col)
            }).collect::<Vec<Cell2048>>()
        }).collect::<Vec<Vec<Cell2048>>>();
        GridCell2048 {
            size,
            cells,
        }
    }

    pub fn shape(&self) -> (u32, u32) {
        (self.size, self.size)
    }
    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        self.iter().filter_map(| cell| {
            cell.is_empty().then(|| (cell.row as usize, cell.col as usize))
        }).collect::<Vec<(usize, usize)>>()
    }

    pub fn insert(&self, row: usize, col:usize, new_cell: Cell2048) {
        self.cells[index] = new_cell;
    }
    pub fn index_to_row_col(&self, index: usize) -> (u32, u32) {
        let row = (index as u32) / self.size;
        let col = (index as u32) % self.size;
        (row, col)
    }
    pub fn iter(&self) -> Iter<'_, Cell2048> {
        self.cells.iter().flat_map(|row|row).collect()
    }

    pub fn enumerate(&self) -> Enumerate<Iter<'_, Cell2048>> {
        self.iter().enumerate()
    }
}