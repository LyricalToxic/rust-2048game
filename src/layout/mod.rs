use std::cell::RefCell;
use std::rc::Rc;

use graphics::{CharacterCache, Context, Graphics, Line};
use graphics::line::Shape::Round;
use rand::{random, thread_rng};
use rand::distributions::uniform::SampleBorrow;
use rand::prelude::SliceRandom;

use crate::console_log_box::ConsoleLogBox;
use crate::constants::{GRID_CELL_SIZE, GRID_LINE_COLOR, GRID_LINE_RADIUS, GRID_OFFSET, P_FOUR_VALUE_CELL};
use crate::game_cell::Cell2048;
use crate::GameState;
use crate::grid_cell::GridCell2048;
use crate::position_grid::PositionGrid;
use crate::score::Score;
use crate::turns_history::TurnsHistory;

pub struct Layout2048 {
    grid: PositionGrid,
    cells: GridCell2048,
    pub game_state: GameState,
    console_log: ConsoleLogBox,
    shape_size: usize,
    score: Score,
    history_stack: TurnsHistory,
}

impl Layout2048 {
    pub fn new(shape_size: usize) -> Self {
        let grid = PositionGrid::new(
            shape_size,
            shape_size,
            GRID_CELL_SIZE,
            Some(GRID_OFFSET),
        );
        let cells = Layout2048::init_cells(shape_size);
        Layout2048 {
            shape_size: shape_size,
            grid: grid,
            cells: cells,
            game_state: GameState::GameInitialized,
            console_log: ConsoleLogBox::new(),
            score: Score::new(),
            history_stack: TurnsHistory::new(5),
        }
    }
    fn init_cells(shape_size: usize) -> GridCell2048 {
        let mut cells = GridCell2048::new(shape_size);
        cells
    }
    pub(crate) fn new_game(&mut self) {
        self.console_log.add_message(String::from("NEW GAME"));
        self.clear_grid();
        self.score = Score::new();
        self.before_moving_event();
        self.add_new_cell(2);
        let possibility: f64 = random();
        if possibility < P_FOUR_VALUE_CELL {
            self.add_new_cell(4);
        }
        self.game_state = GameState::InGame;
        self.after_moving_event();
    }
    pub(crate) fn spawn_2048(&mut self) {
        match self.game_state {
            GameState::InGame => {
                self.add_new_cell(2048);
            }
            _ => {}
        }
    }
    fn clear_grid(&mut self) {
        self.cells = Layout2048::init_cells(self.shape_size)
    }

    fn add_new_cell(&mut self, value: usize) -> bool {
        if self.is_field_fulfilled() {
            return false;
        }
        self.score.inc(value as u64);

        let empty_cells = self.cells.get_empty_cells();
        let (chosen_row, chosen_col) = *empty_cells.choose(&mut thread_rng()).unwrap();
        let new_cell = Cell2048::new(value, chosen_row, chosen_col, None, 10);
        self.cells.insert(&new_cell);
        let message =
            format!("Added cell with value: {value}, row = {chosen_row} col = {chosen_col}");
        self.console_log.add_message(message);
        return true;
    }

    pub(crate) fn draw<C, G>(&mut self, c: Context, gl: &mut G, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let line = Line {
            color: GRID_LINE_COLOR,
            radius: GRID_LINE_RADIUS,
            shape: Round,
        };
        self.grid.draw(&line, &c.draw_state, c.transform, gl);
        for (row, col, cell) in self.cells.enumerate() {
            let (pos_x, pos_y) = self.grid.get_pos((col, row));
            cell.borrow().draw(pos_x, pos_y, GRID_CELL_SIZE, c, gl, glyph_cache);
        }
        self.console_log.draw(c, gl, glyph_cache);

        let grid_rectangle = self.grid.get_rectangle_pos();
        let score_pos_x = grid_rectangle[0] + (grid_rectangle[2] - grid_rectangle[0]) / 2.0;
        let score_pos_y = grid_rectangle[3] + 50.0;
        self.score.draw_pos(score_pos_x, score_pos_y, c, gl, c.transform, glyph_cache);
    }
    pub fn update(&mut self) {
        for cell in self.cells.flat_iter() {
            cell.borrow_mut().update();
        }
    }
    pub(crate) fn move_right(&mut self) {
        match self.game_state {
            GameState::InGame => {
                self.before_moving_event();
                for row in self.cells.rows() {
                    for cell in row.iter().rev() {
                        let mut current_col = cell.borrow().col;
                        let mut next_col = current_col + 1;
                        while next_col < self.shape_size {
                            let current_cell = self.cells.get_cell(cell.borrow().row, current_col);
                            let next_cell = self.cells.get_cell(cell.borrow().row, next_col);
                            self.move_cell(
                                &mut current_cell.borrow_mut(),
                                &mut next_cell.borrow_mut(),
                            );
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
                self.after_moving_event()
            }
            _ => {}
        }
    }
    pub(crate) fn move_down(&mut self) {
        match self.game_state {
            GameState::InGame => {
                self.before_moving_event();
                for row in self.cells.rows().rev() {
                    for cell in row.iter() {
                        let mut current_row = cell.borrow().row;
                        let mut next_row = current_row + 1;
                        while next_row < self.shape_size {
                            let current_cell = self.cells.get_cell(current_row, cell.borrow().col);
                            let next_cell = self.cells.get_cell(next_row, cell.borrow().col);
                            self.move_cell(
                                &mut current_cell.borrow_mut(),
                                &mut next_cell.borrow_mut(),
                            );
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
                self.after_moving_event()
            }
            _ => {}
        }
    }
    pub(crate) fn move_left(&mut self) {
        match self.game_state {
            GameState::InGame => {
                self.before_moving_event();
                for row in self.cells.rows() {
                    for cell in row.iter() {
                        let mut current_col = cell.borrow().col;
                        let mut prev_col = current_col as isize - 1;
                        while prev_col >= 0 {
                            let current_cell = self.cells.get_cell(cell.borrow().row, current_col);
                            let next_cell =
                                self.cells.get_cell(cell.borrow().row, prev_col as usize);
                            self.move_cell(
                                &mut current_cell.borrow_mut(),
                                &mut next_cell.borrow_mut(),
                            );
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
                self.after_moving_event()
            }
            _ => {}
        }
    }
    pub(crate) fn move_up(&mut self) {
        match self.game_state {
            GameState::InGame => {
                self.before_moving_event();
                for row in self.cells.rows() {
                    for cell in row.iter() {
                        let mut current_row = cell.borrow().row;
                        let mut prev_row = current_row as isize - 1;
                        while prev_row >= 0 {
                            let current_cell = self.cells.get_cell(current_row, cell.borrow().col);
                            let next_cell =
                                self.cells.get_cell(prev_row as usize, cell.borrow().col);
                            self.move_cell(
                                &mut current_cell.borrow_mut(),
                                &mut next_cell.borrow_mut(),
                            );
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
                self.after_moving_event()
            }
            _ => {}
        }
    }

    fn before_moving_event(&mut self) {
        self.history_stack.add(self.cells.clone());
    }
    fn after_moving_event(&mut self) {
        for cell in self.cells.flat_iter() {
            cell.borrow_mut().update_background();
        }
    }

    fn move_cell(&self, current_cell: &mut Cell2048, next_cell: &mut Cell2048) {
        if current_cell.is_empty() {
            return;
        }
        if next_cell.is_empty() {
            next_cell.devour_soft(current_cell);
            current_cell.reset();
            return;
        }
        if current_cell == next_cell {
            let accumulated_value = next_cell.accumulate(current_cell);
            self.score.inc(accumulated_value as u64);
            current_cell.reset();
            return;
        }
    }
    fn is_field_fulfilled(&self) -> bool {
        !self
            .cells
            .flat_iter()
            .any(|cell: Rc<RefCell<Cell2048>>| cell.borrow().is_empty())
    }

    pub(crate) fn check_game_over(&mut self) {
        if self.is_game_over() {
            self.game_state = GameState::GameOver;
            self.console_log.add_message(String::from("Game over"));
        }
    }
    pub(crate) fn check_game_won(&mut self) {
        if self.is_game_won() {
            self.game_state = GameState::GameWon;
            self.console_log.add_message(String::from(format!("Congratulations! You win. Score: {}", self.score.get())));
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
    fn is_game_won(&self) -> bool {
        self.cells.flat_iter().any(|cell2048: Rc<RefCell<Cell2048>>| {
            *cell2048.borrow().value == 2048
        })
    }
    pub fn turn_back(&mut self) {
        match self.game_state {
            GameState::InGame => {
                if let Some(previous_grid_cell) = self.history_stack.pop() {
                    self.console_log.add_message(String::from(format!("Moved back. Left {} turns", self.history_stack.size())));
                    self.devour(*previous_grid_cell);
                } else {
                    self.console_log.add_message(String::from("Can not moved back"));
                }
            }
            _ => { self.console_log.add_message(String::from("Press whitespace to start the game")) }
        }
    }
    fn devour(&mut self, grid_cell2048: GridCell2048) {
        self.cells = grid_cell2048
    }
}
