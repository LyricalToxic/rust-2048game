use glutin_window::OpenGL;
use graphics::clear;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::{Button, ButtonArgs, ButtonState, Key, RenderArgs, UpdateArgs};

use crate::{Direction, GameState};
use crate::constants::WINDOW_BACKGROUND_COLOR;
use crate::layout::Layout2048;

pub struct GameField {
    backend: GlGraphics,
    layout: Layout2048,
    direction: Direction,
    press_time: u32,
}

impl GameField {
    pub fn new() -> Self {
        let opengl = OpenGL::V4_5;
        let backend = GlGraphics::new(opengl);
        let shape_size = 4;

        let layout2048 = Layout2048::new(shape_size);
        GameField {
            backend,
            layout: layout2048,
            direction: Direction::Release,
            press_time: 0,
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {
        let mut glyph_cache = GlyphCache::new(
            "./assets/fonts/FiraSans-Regular.ttf",
            (),
            TextureSettings::new(),
        )
            .unwrap();

        self.backend.draw(args.viewport(), |c, gl| {
            clear(WINDOW_BACKGROUND_COLOR, gl);
            self.layout.draw(c, gl, &mut glyph_cache);
        });
    }
    pub fn update(&mut self, _args: UpdateArgs) {
        let press_frequency = 15;
        let should_move = self.press_time == 0
            || (self.press_time > 100 && self.press_time % press_frequency == 0);
        match self.direction {
            Direction::Right => {
                if should_move
                {
                    self.layout.move_right();
                }
                self.press_time += 1;
            }
            Direction::Down => {
                if should_move
                {
                    self.layout.move_down();
                }
                self.press_time += 1;
            }
            Direction::Up => {
                if should_move
                {
                    self.layout.move_up();
                }
                self.press_time += 1;
            }
            Direction::Left => {
                if should_move
                {
                    self.layout.move_left();
                }
                self.press_time += 1;
            }
            Direction::Release => {
                self.press_time = 0;
            }
        }

        match self.layout.game_state {
            GameState::GameOver => {}
            GameState::GameInitialized => {}
            GameState::GameWon => {}
            GameState::InGame => {
                self.layout.check_game_over();
                self.layout.check_game_won();
            }
        }
        self.layout.update()
    }
    pub fn on_button(&mut self, button: &ButtonArgs) {
        match (button.button, button.state) {
            (Button::Keyboard(Key::Right), ButtonState::Press) => {
                self.direction = Direction::Right;
            }
            (Button::Keyboard(Key::Right), ButtonState::Release) => {
                self.direction = Direction::Release;
            }
            (Button::Keyboard(Key::Left), ButtonState::Press) => {
                self.direction = Direction::Left;
            }
            (Button::Keyboard(Key::Left), ButtonState::Release) => {
                self.direction = Direction::Release;
            }
            (Button::Keyboard(Key::Down), ButtonState::Press) => {
                self.direction = Direction::Down;
            }
            (Button::Keyboard(Key::Down), ButtonState::Release) => {
                self.direction = Direction::Release;
            }
            (Button::Keyboard(Key::Up), ButtonState::Press) => {
                self.direction = Direction::Up;
            }
            (Button::Keyboard(Key::Up), ButtonState::Release) => {
                self.direction = Direction::Release;
            }
            (Button::Keyboard(Key::Space), ButtonState::Press) => {
                self.layout.new_game();
            }
            (Button::Keyboard(Key::W), ButtonState::Press) => {
                self.layout.spawn_2048();
            }
            (Button::Keyboard(Key::Backspace), ButtonState::Press) => {
                self.layout.turn_back();
            }
            _ => {}
        }
    }
}
