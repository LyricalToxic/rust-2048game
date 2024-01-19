#![feature(inherent_associated_types)]

use std::cmp::max;
use std::collections::HashMap;

use graphics::{CharacterCache, clear, Context, Graphics, Line, rectangle, Text};
use graphics::color::{BLACK, GREEN, WHITE};
use graphics::grid::Grid;
use graphics::line::Shape;
use graphics::math::Scalar;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::GlyphCache;
use piston::{Button, ButtonState};
use piston::input::{ButtonArgs, RenderArgs, UpdateArgs};
use piston::Key;
use piston_window::prelude::*;
use rand::{random, Rng, thread_rng};

use game_field::game_field::GameField;

pub mod grid_cell;
pub mod game_field;
pub mod game_cell;
pub mod layout;

pub struct Point {
    pub x: Scalar,
    pub y: Scalar,
}

enum Direction {
    Left,
    Right,
    Down,
    Up,
    Release,
}


enum GameState {
    GameOver,
    InGame,
    GameInitialized,
}

struct ConsoleLogLayout2048 {
    messages: Vec<String>,
    limit: u32,
}

impl ConsoleLogLayout2048 {
    fn new() -> Self {
        ConsoleLogLayout2048 {
            messages: vec![],
            limit: 30,
        }
    }
    fn draw<C, G>(&self, c: Context, gl: &mut G, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let rec_x = 550.0;
        let rec_y = 50.0;
        let width = 300.0;
        let height = 600.0;
        let font_size = 16;
        let intend_x = 10;
        let intend_y = 2;
        let mut stack_pointer_x = intend_x as f64 + rec_x as f64;
        let mut stack_pointer_y = rec_y as f64 + height as f64 - intend_y as f64;
        rectangle(WHITE, [rec_x, rec_y, width, height], c.transform, gl);
        if self.messages.is_empty() {
            return;
        }
        let min_value: i64 = self.messages.len() as i64 - self.limit as i64;
        let start_pos: usize = max(min_value, 0) as usize;
        for message in self.messages[start_pos..].iter().rev() {
            Text::new_color(BLACK, font_size)
                .draw_pos(
                    &*message.clone(),
                    [stack_pointer_x, stack_pointer_y],
                    glyph_cache,
                    &c.draw_state,
                    c.transform,
                    gl,
                )
                .unwrap();
            stack_pointer_y = stack_pointer_y - intend_y as f64 - font_size as f64 - intend_y as f64;
        }
    }
    fn add_message(&mut self, message: String) {
        self.messages.push(message)
    }
}




