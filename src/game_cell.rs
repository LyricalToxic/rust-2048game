use std::any::Any;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::ops::Add;

use graphics::{CharacterCache, Context, Graphics, Text};
use graphics::color::RED;
use graphics::types::Scalar;
use rand::distributions::uniform::SampleBorrow;

pub(crate) mod game_cell;

const FONT_SIZE: u32 = 32;

#[derive(Clone, Debug)]
pub struct Cell2048 {
    pub(crate) value: Box<usize>,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Cell2048
{
    pub(crate) fn new(value: usize, row: usize, col: usize) -> Self {
        Cell2048 {
            value: Box::new(value),
            row: row,
            col: col,
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        *self.value == 0
    }
    pub(crate) fn draw<C, G>(&self, pos_x: Scalar, pos_y: Scalar, grid_cell_size: f64, c: Context, gl: &mut G, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let str_value = self.to_string();
        let value_length = str_value.len();
        let cell_center_x: Scalar = (pos_x + (grid_cell_size / 2.0) - (value_length as Scalar * FONT_SIZE as Scalar / 4.0));
        let cell_center_y: Scalar = (pos_y + (grid_cell_size / 2.0) + (FONT_SIZE as Scalar / 4.0));
        Text::new_color(RED, FONT_SIZE).draw_pos(
            &str_value,
            [cell_center_x, cell_center_y],
            glyph_cache,
            &c.draw_state,
            c.transform,
            gl,
        )
            .unwrap();
    }
    fn to_string(&self) -> String {
        if *self.value == 0 {
            return String::from("");
        }
        String::from(&self.value.to_string())
    }
    pub fn devour(&mut self, cell: &Cell2048) {
        self.value = Box::new(*cell.value);
    }
    pub fn accumulate(&mut self, cell: &Cell2048) {
        self.value = Box::new(*self.value + *cell.value);
    }
    pub fn reset(&mut self) {
        self.value = Box::new(0);
    }
}

impl Add for Cell2048 {
    type Output = Box<usize>;

    fn add(self, rhs: Self) -> Self::Output {
        Box::new(*self.value + *rhs.value)
    }
}

impl PartialEq for Cell2048 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}


impl PartialOrd for Cell2048 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}