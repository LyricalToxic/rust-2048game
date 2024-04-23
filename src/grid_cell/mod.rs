use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::slice::Iter;

use crate::game_cell::Cell2048;

#[derive(Debug)]
pub struct GridCell2048 {
    size: usize,
    cells: Vec<Vec<Rc<RefCell<Cell2048>>>>,
}

pub struct GridCellIterator2048<'a> {
    iter_row: usize,
    iter_col: usize,
    grid_cell2048: &'a GridCell2048,
}

pub struct GridCellEnumerate2048<'a> {
    iter_row: usize,
    iter_col: usize,
    grid_cell2048: &'a GridCell2048,
}

impl<'a> Iterator for GridCellIterator2048<'a> {
    type Item = Rc<RefCell<Cell2048>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_row == self.grid_cell2048.size && self.iter_col == 0 {
            return None;
        }
        let next_item = Rc::clone(
            self.grid_cell2048
                .cells
                .get(self.iter_row)
                .unwrap()
                .get(self.iter_col)
                .unwrap(),
        );
        self.iter_col += 1;
        if self.iter_col >= self.grid_cell2048.size {
            self.iter_col = 0;
            self.iter_row += 1;
        }
        Some(next_item)
    }
}

impl<'a> Iterator for GridCellEnumerate2048<'a> {
    type Item = (usize, usize, Rc<RefCell<Cell2048>>);

    fn next(&mut self) -> Option<Self::Item> {
        let current_row = self.iter_row.clone();
        let current_col = self.iter_col.clone();
        if self.iter_row == self.grid_cell2048.size && self.iter_col == 0 {
            return None;
        }
        let next_item = Rc::clone(
            self.grid_cell2048
                .cells
                .get(self.iter_row)
                .unwrap()
                .get(self.iter_col)
                .unwrap(),
        );
        self.iter_col += 1;
        if self.iter_col >= self.grid_cell2048.size {
            self.iter_col = 0;
            self.iter_row += 1;
        }
        Option::from((current_row, current_col, next_item))
    }
}

impl GridCell2048 {
    pub fn new(size: usize) -> Self {
        let cells = (0..size)
            .map(|row| {
                (0..size)
                    .map(|col| Rc::new(RefCell::new(Cell2048::new(0, row, col, None))))
                    .collect::<Vec<Rc<RefCell<Cell2048>>>>()
            })
            .collect::<Vec<Vec<Rc<RefCell<Cell2048>>>>>();
        GridCell2048 { size, cells }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.size, self.size)
    }
    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        self.flat_iter()
            .filter_map(|cell| {
                cell.borrow()
                    .is_empty()
                    .then(|| (cell.borrow().row, cell.borrow().col))
            })
            .collect::<Vec<(usize, usize)>>()
    }

    pub fn insert(&mut self, new_cell: &Cell2048) {
        self.cells[new_cell.row][new_cell.col]
            .borrow_mut()
            .devour(new_cell);
    }
    pub fn flat_iter(&self) -> GridCellIterator2048<'_> {
        GridCellIterator2048 {
            iter_row: 0,
            iter_col: 0,
            grid_cell2048: self,
        }
    }
    pub fn rows(&self) -> Iter<'_, Vec<Rc<RefCell<Cell2048>>>> {
        self.cells.iter()
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Rc<RefCell<Cell2048>> {
        Rc::clone(&self.cells[row][col])
    }

    pub fn enumerate(&self) -> GridCellEnumerate2048<'_> {
        GridCellEnumerate2048 {
            iter_row: 0,
            iter_col: 0,
            grid_cell2048: self,
        }
    }
    pub fn get(&self, row: usize, col: usize) -> Rc<RefCell<Cell2048>> {
        Rc::clone(&self.cells.get(row).unwrap().get(col).unwrap())
    }
}

impl Display for GridCell2048 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            for cell in row {
                write!(f, "{}", cell.borrow().value).expect("");
            }
            writeln!(f).expect("");
        }
        Ok(())
    }
}
