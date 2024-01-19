use glutin_window::OpenGL;
use graphics::clear;
use graphics::color::GREEN;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::{Button, ButtonArgs, ButtonState, Key, RenderArgs, UpdateArgs};
use crate::{Direction, GameState};
use crate::layout::layout::Layout2048;
pub struct GameField {
    backend: GlGraphics,
    layout: Layout2048,
    direction: Direction,
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
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {
        let mut glyph_cache = GlyphCache::new("./assets/fonts/FiraSans-Regular.ttf", (), TextureSettings::new()).unwrap();

        self.backend.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            self.layout.draw(c, gl, &mut glyph_cache);
        });
    }
    pub fn update(&mut self, args: UpdateArgs) {
        match self.direction {
            Direction::Right => {
                self.layout.move_right();
                self.direction = Direction::Release;
            }
            Direction::Down => {
                self.layout.move_down();
                self.direction = Direction::Release;
            }
            Direction::Up => {
                self.layout.move_up();
                self.direction = Direction::Release;
            }
            Direction::Left => {
                self.layout.move_left();
                self.direction = Direction::Release;
            }
            Direction::Release => {}
        }

        match self.layout.game_state {
            GameState::GameOver => {}
            GameState::GameInitialized => {}
            GameState::InGame => {
                self.layout.check_game_over();
            }
        }
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
            _ => {}
        }
    }
}
