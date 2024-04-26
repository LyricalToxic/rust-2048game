use std::collections::VecDeque;

use crate::grid_cell::GridCell2048;

pub struct TurnsHistory {
    deep: u8,
    stack: VecDeque<Box<GridCell2048>>,
}

impl TurnsHistory {
    pub fn new(deep: u8) -> Self {
        TurnsHistory {
            deep: deep,
            stack: VecDeque::with_capacity(deep as usize),
        }
    }
    pub fn pop(&mut self) -> Option<Box<GridCell2048>> {
        self.stack.pop_back()
    }
    pub fn add(&mut self, grid_cell2048: GridCell2048) {
        if self.stack.len() == self.deep as usize {
            self.stack.pop_front();
        }
        self.stack.push_back(Box::new(grid_cell2048));
    }
    pub fn size(&self) -> usize {
        self.stack.len()
    }
}