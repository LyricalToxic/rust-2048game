use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::ops::Add;

use graphics::{CharacterCache, Context, Graphics, Rectangle, Text};
use graphics::color::{hex, RED};
use graphics::rectangle::square;
use graphics::types::{Color, Scalar};
use once_cell::sync::Lazy;
use rand::distributions::uniform::SampleBorrow;

use crate::constants::{CELL_BACKGROUND_COLOR, GRID_LINE_RADIUS, TEXT_FONT_SIZE};

static CELL_VALUES_COLORS: Lazy<HashMap<usize, Color>> = Lazy::new(|| {
    HashMap::from([
        (0, CELL_BACKGROUND_COLOR),
        (2, hex("EDE342")),
        (4, hex("F2BF6C")),
        (8, hex("F69A97")),
        (16, hex("FB76C1")),
        (32, hex("FF51EB")),
        (64, hex("58EFEC")),
        (128, hex("7CCAD5")),
        (256, hex("A0A6BE")),
        (512, hex("C481A7")),
        (1024, hex("E85C90")),
        (2048, hex("9F6976")),
    ])
});

#[derive(Clone, Debug)]
pub struct Cell2048 {
    pub value: Box<usize>,
    pub row: usize,
    pub col: usize,
    pub background: Color,
    pub padding: Scalar,
}

impl Cell2048 {
    pub(crate) fn new(value: usize, row: usize, col: usize, background: Option<Color>) -> Self {
        let background_color = background.unwrap_or_else(|| CELL_BACKGROUND_COLOR);
        Cell2048 {
            value: Box::new(value),
            row: row,
            col: col,
            background: background_color,
            padding: 3.0 as Scalar,
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        *self.value == 0
    }
    pub(crate) fn draw<C, G>(
        &self,
        pos_x: Scalar,
        pos_y: Scalar,
        grid_cell_size: f64,
        c: Context,
        gl: &mut G,
        glyph_cache: &mut C,
    ) where
        C: CharacterCache,
        G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let background_center_x: Scalar = (pos_x + self.padding);
        let background_center_y: Scalar = (pos_y + self.padding);
        Rectangle::new_round(self.background, GRID_LINE_RADIUS)
            .draw_tri(
                square(
                    background_center_x,
                    background_center_y,
                    grid_cell_size - self.padding * 2.0,
                ),
                &c.draw_state,
                c.transform,
                gl,
            );

        let str_value = self.to_string();
        let value_length = str_value.len();
        let text_center_x: Scalar = pos_x + (grid_cell_size / 2.0)
            - (value_length as Scalar * TEXT_FONT_SIZE as Scalar / 4.0);
        let text_center_y: Scalar =
            pos_y + (grid_cell_size / 2.0) + (TEXT_FONT_SIZE as Scalar / 4.0);
        Text::new_color(RED, TEXT_FONT_SIZE)
            .draw_pos(
                &str_value,
                [text_center_x, text_center_y],
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
        self.background = cell.background;
    }
    pub fn devour_soft(&mut self, cell: &Cell2048) {
        self.value = Box::new(*cell.value);
    }
    pub fn accumulate(&mut self, cell: &Cell2048) -> usize {
        self.value = Box::new(*self.value + *cell.value);
        *self.value.borrow()
    }
    pub fn reset(&mut self) {
        self.value = Box::new(0);
    }

    fn background(&mut self, color: Color) {
        self.background = color;
    }
    pub fn update_background(&mut self) {
        self.background(CELL_VALUES_COLORS.get(self.value.borrow()).unwrap().clone());
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
