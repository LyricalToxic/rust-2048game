extern crate game2048;

use opengl_graphics::*;
use piston::event_loop::{Events, EventSettings};
use piston::input::{ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;
use game2048::game_field::game_field::GameField;

fn main() {
    let mut window: Sdl2Window = WindowSettings::new("game2048", [900, 900])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game_field = GameField::new();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game_field.render(&args);
        };
        if let Some(args) = e.update_args() {
            game_field.update(args);
        };

        if let Some(args) = e.button_args() {
            game_field.on_button(&args);
        };
    }
}