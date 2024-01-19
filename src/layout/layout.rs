use graphics::{CharacterCache, Context, Graphics, Line};
use graphics::color::WHITE;
use graphics::grid::Grid;
use graphics::line::Shape;
use rand::{random, thread_rng};
use rand::prelude::SliceRandom;

use crate::{ConsoleLogLayout2048, GameState};
use crate::game_cell::Cell2048;
use crate::grid_cell::grid_cell::GridCell2048;

pub struct Layout2048 {
    grid: Grid,
    cells: GridCell2048,
    pub game_state: GameState,
    console_log: ConsoleLogLayout2048,
    shape_size: u32,
}

impl Layout2048 {
    pub fn new(shape_size: u32) -> Self {
        let grid = Grid {
            cols: shape_size,
            rows: shape_size,
            units: 100.0,
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
    fn init_cells(shape_size: u32) -> GridCell2048 {
        let mut cells = GridCell2048::new(shape_size);
        cells
    }
    pub(crate) fn new_game(&mut self) {
        self.console_log.add_message(String::from("NEW GAME"));
        self.clear_grid();

        self.add_new_cell(&String::from("2"));
        let possibility: f64 = random();
        if possibility < 0.2 {
            self.add_new_cell(&String::from("4"));
        }
        self.game_state = GameState::InGame;
    }
    fn clear_grid(&mut self) {
        self.cells = Layout2048::init_cells(self.shape_size)
    }

    fn add_new_cell(&mut self, value: &String) -> bool {
        if self.is_field_fulfilled() {
            return false;
        }
        let new_cell = Cell2048::new(&String::from(value));
        let empty_cells = self.cells.get_empty_cells();
        let chosen_cell_index = *empty_cells.choose(&mut thread_rng()).unwrap();

        self.cells.insert(chosen_cell_index, new_cell);
        let (row, col) = self.cells.index_to_row_col(chosen_cell_index);
        let message = format!("Added cell with value: {value}, row = {row} col = {col}");
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
        for (index, cell) in self.cells.enumerate() {
            let (row, col) = self.cells.index_to_row_col(index);
            cell.draw(row, col, c, gl, glyph_cache);
        }
        self.console_log.draw(c, gl, glyph_cache)
    }
    pub(crate) fn move_right(&mut self) {
        match self.game_state {
            GameState::InGame => {

                self.move_cell(row, col, row, col + 1);
                self.add_new_cell(&String::from("2"));
            }
            _ => {}
        }
    }
    pub(crate) fn move_down(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for row in 0..self.shape_size - 1 {
                    for col in 0..self.shape_size {
                        self.move_cell(row, col, row + 1, col);
                    }
                }
                self.add_new_cell(&String::from("2"));
            }
            _ => {}
        }
    }
    pub(crate) fn move_left(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for col in (1..self.shape_size).rev() {
                    for row in 0..self.shape_size {
                        self.move_cell(row, col, row, col - 1);
                    }
                }
                self.add_new_cell(&String::from("2"));
            }
            _ => {}
        }
    }
    pub(crate) fn move_up(&mut self) {
        match self.game_state {
            GameState::InGame => {
                for row in (1..self.shape_size).rev() {
                    for col in 0..self.shape_size {
                        self.move_cell(row, col, row - 1, col);
                    }
                }
                self.add_new_cell(&String::from("2"));
            }
            _ => {}
        }
    }

    fn move_cell(&mut self, current_row: u32, current_col: u32, next_row: u32, next_col: u32) {
        let current_cell = self.cells.get(&(current_row, current_col)).unwrap().clone();
        let next_cell = self.cells.get_mut(&(next_row, next_col)).unwrap().clone();
        if !current_cell.is_empty() && current_cell.value == next_cell.value {
            let sum = next_cell.value.parse::<u32>().unwrap() + current_cell.value.parse::<u32>().unwrap();
            self.cells.insert((current_row, current_col), Cell2048::new(&String::from("")));
            self.cells.insert((next_row, next_col), Cell2048::new(&sum.to_string()));
        } else if !current_cell.is_empty() && next_cell.value.is_empty() {
            self.cells.insert((current_row, current_col), Cell2048::new(&String::from("")));
            self.cells.insert((next_row, next_col), Cell2048::new(&current_cell.value.clone()));
        }
    }
    fn print_cells(&self) {
        for (pos, cell) in self.cells.iter() {
            println!("pos ={:?}, cell = {:?}", pos, cell);
        }
    }
    fn is_field_fulfilled(&self) -> bool {
        !self.cells.iter().any(|(_pos, cell)| {
            cell.is_empty()
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
                let current_cell = self.cells.get(&(row, col)).unwrap();
                if let Some(right_cell) = self.cells.get(&(row, col + 1)) {
                    if right_cell.value == current_cell.value {
                        return false;
                    }
                }
                if let Some(bottom_cell) = self.cells.get(&(row - 1, col)) {
                    if bottom_cell.value == current_cell.value {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}