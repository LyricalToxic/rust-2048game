use graphics::{CharacterCache, Graphics};
use graphics::math::Scalar;
use piston_window::prelude::*;
use rand::Rng;

pub mod constants;
pub mod game_cell;
pub mod game_field;
pub mod grid_cell;
pub mod layout;
pub mod position_grid;
pub mod text_container;
pub mod console_log_box;
pub mod score;
pub mod turns_history;

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
    GameWon,
    InGame,
    GameInitialized,
}

