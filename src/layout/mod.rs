use std::cell::RefCell;
use std::rc::Rc;

use graphics::{CharacterCache, Context, Graphics, Line};
use graphics::color::WHITE;
use graphics::grid::Grid;
use graphics::line::Shape;
use rand::{random, thread_rng};
use rand::prelude::SliceRandom;

use crate::{ConsoleLogLayout2048, GameState};
use crate::game_cell::Cell2048;
use crate::grid_cell::GridCell2048;

const GRID_CELL_SIZE: f64 = 100.0;
const P_FOUR_VALUE_CELL: f64 = 0.1;

pub struct Layout2048 {
    grid: Grid,
    cells: GridCell2048,
    pub game_state: GameState,
    console_log: ConsoleLogLayout2048,
    shape_size: usize,
}

impl Layout2048 {
    pub fn new(shape_size: usize) -> Self {
        let grid = Grid {
            cols: shape_size as u32,
            rows: shape_size as u32,
            units: GRID_CELL_SIZE,
        };
        let cells = Layout2048::init_cells(shape_size);
        Layout2048 {
            shape_size: shape_size,
            grid: grid,
            cells: cells,
            game_state: GameState::GameInitialized,
            console_log: ConsoleLogLayout2048::new(),
        }
    }
    fn init_cells(shape_size: usize) -> GridCell2048 {
        let mut cells = GridCell2048::new(shape_size);
        cells
    }
    pub(crate) fn new_game(&mut self) {
        self.console_log.add_message(String::from("NEW GAME"));
        self.clear_grid();

        self.add_new_cell(2);
        let possibility: f64 = random();
        if possibility < P_FOUR_VALUE_CELL {
            self.add_new_cell(4);
        }
        self.game_state = GameState::InGame;
    }
    fn clear_grid(&mut self) {
        self.cells = Layout2048::init_cells(self.shape_size)
    }

    fn add_new_cell(&mut self, value: usize) -> bool {
        if self.is_field_fulfilled() {
            return false;
        }

        let empty_cells = self.cells.get_empty_cells();
        let (chosen_row, chosen_col) = *empty_cells.choose(&mut thread_rng()).unwrap();
        let new_cell = Cell2048::new(value, chosen_row, chosen_col);
        self.cells.insert(&new_cell);
        let message = format!("Added cell with value: {value}, row = {chosen_row} col = {chosen_col}");
        self.console_log.add_message(message);
        return true;
    }

    pub(crate) fn draw<C, G>(&mut self, c: Context, gl: &mut G, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let line = Line { color: WHITE, radius: 1.0, shape: Shape::Square };
        self.grid.draw(&line, &c.draw_state, c.transform, gl);
        for (row, col, cell) in self.cells.enumerate() {
            let pos_x = self.grid.x_pos((col as u32, row as u32));
            let pos_y = self.grid.y_pos((col as u32, row as u32));
            cell.borrow().draw(pos_x, pos_y, GRID_CELL_SIZE, c, gl, glyph_cache);
        }
        self.console_log.draw(c, gl, glyph_cache)
    }
    pub(crate) fn move_right(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for row in self.cells.rows() {
                    for cell in row.iter().rev() {
                        let mut current_col = cell.borrow().col;
                        let mut next_col = current_col + 1;
                        while next_col < self.shape_size {
                            let current_cell = self.cells.get_cell(cell.borrow().row, current_col);
                            let next_cell = self.cells.get_cell(cell.borrow().row, next_col);
                            self.move_cell(&mut current_cell.borrow_mut(), &mut next_cell.borrow_mut());
                            current_col = next_col;
                            next_col = current_col + 1;
                        }
                    }
                }
                self.add_new_cell(2);
                let possibility: f64 = random();
                if possibility < P_FOUR_VALUE_CELL {
                    self.add_new_cell(4);
                }
            }
            _ => {}
        }
    }
    pub(crate) fn move_down(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for row in self.cells.rows().rev() {
                    for cell in row.iter() {
                        let mut current_row = cell.borrow().row;
                        let mut next_row = current_row + 1;
                        while next_row < self.shape_size {
                            let current_cell = self.cells.get_cell(current_row, cell.borrow().col);
                            let next_cell = self.cells.get_cell(next_row, cell.borrow().col);
                            self.move_cell(&mut current_cell.borrow_mut(), &mut next_cell.borrow_mut());
                            current_row = next_row;
                            next_row = current_row + 1;
                        }
                    }
                }
                self.add_new_cell(2);
                let possibility: f64 = random();
                if possibility < P_FOUR_VALUE_CELL {
                    self.add_new_cell(4);
                }
            }
            _ => {}
        }
    }
    pub(crate) fn move_left(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for row in self.cells.rows() {
                    for cell in row.iter() {
                        let mut current_col = cell.borrow().col;
                        let mut prev_col = current_col as isize - 1;
                        while prev_col >= 0 {
                            let current_cell = self.cells.get_cell(cell.borrow().row, current_col);
                            let next_cell = self.cells.get_cell(cell.borrow().row, prev_col as usize);
                            self.move_cell(&mut current_cell.borrow_mut(), &mut next_cell.borrow_mut());
                            current_col = prev_col as usize;
                            prev_col = current_col as isize - 1;
                        }
                    }
                }
                self.add_new_cell(2);
                let possibility: f64 = random();
                if possibility < P_FOUR_VALUE_CELL {
                    self.add_new_cell(4);
                }
            }
            _ => {}
        }
    }
    pub(crate) fn move_up(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for row in self.cells.rows() {
                    for cell in row.iter() {
                        let mut current_row = cell.borrow().row;
                        let mut prev_row = current_row as isize - 1;
                        while prev_row >= 0 {
                            let current_cell = self.cells.get_cell(current_row, cell.borrow().col);
                            let next_cell = self.cells.get_cell(prev_row as usize, cell.borrow().col);
                            self.move_cell(&mut current_cell.borrow_mut(), &mut next_cell.borrow_mut());
                            current_row = prev_row as usize;
                            prev_row = current_row as isize - 1;
                        }
                    }
                }
                self.add_new_cell(2);
                let possibility: f64 = random();
                if possibility < P_FOUR_VALUE_CELL {
                    self.add_new_cell(4);
                }
            }
            _ => {}
        }
    }

    fn move_cell(&self, current_cell: &mut Cell2048, next_cell: &mut Cell2048) {
        if current_cell.is_empty() {
            return;
        }
        if next_cell.is_empty() {
            next_cell.devour(current_cell);
            current_cell.reset();
            return;
        }
        if current_cell == next_cell {
            next_cell.accumulate(current_cell);
            current_cell.reset();
            return;
        }
    }
    fn print_cells(&self) {
        for cell in self.cells.flat_iter() {
            println!("cell = {:?}", cell);
        }
    }
    fn is_field_fulfilled(&self) -> bool {
        !self.cells.flat_iter().any(|cell: Rc<RefCell<Cell2048>>| {
            cell.borrow().is_empty()
        })
    }

    pub(crate) fn check_game_over(&mut self) {
        if self.is_game_over() {
            self.game_state = GameState::GameOver;
            self.console_log.add_message(String::from("Game over"));
        }
    }

    fn is_game_over(&self) -> bool {
        if !self.is_field_fulfilled() {
            return false;
        };
        for row in 0..self.shape_size {
            for col in 0..self.shape_size {
                let current_cell = self.cells.get(row, col);
                if col + 1 != self.shape_size {
                    if let right_cell = self.cells.get(row, col + 1).borrow() {
                        if right_cell.value == current_cell.borrow().value {
                            return false;
                        }
                    }
                }
                if row + 1 != self.shape_size {
                    if let bottom_cell = self.cells.get(row + 1, col).borrow() {
                        if bottom_cell.value == current_cell.borrow().value {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }
}