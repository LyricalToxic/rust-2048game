use std::cmp::max;

use graphics::{CharacterCache, Context, Graphics, Rectangle, Text, Transformed};
use graphics::color::{BLACK, hex, WHITE};
use graphics::types::Scalar;

use crate::constants::GRID_LINE_RADIUS;
use crate::text_container::TextContainer;

pub struct ConsoleLogBox {
    messages: Vec<String>,
    limit: u32,
}

impl ConsoleLogBox {
    pub(crate) fn new() -> Self {
        ConsoleLogBox {
            messages: vec![],
            limit: 30,
        }
    }
    pub(crate) fn draw<C, G>(&self, c: Context, gl: &mut G, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let rec_x = 550.0;
        let rec_y = 50.0;
        let width = 300.0;
        let height = 600.0;
        let font_size = 16;
        let intend_x: Scalar = 10.0;
        let intend_y: Scalar = 10.0;
        let mut stack_pointer_x = intend_x + rec_x;
        let mut stack_pointer_y = rec_y + height;
        Rectangle::new_round_border(WHITE, GRID_LINE_RADIUS, GRID_LINE_RADIUS).color(hex("CD9632"))
            .draw([rec_x, rec_y, width, height], &c.draw_state, c.transform, gl);
        if self.messages.is_empty() {
            return;
        }
        let min_value: i64 = self.messages.len() as i64 - self.limit as i64;
        let start_pos: usize = max(min_value, 0) as usize;
        for message in self.messages[start_pos..].iter().rev() {
            let message_container = TextContainer::new(
                Text::new_color(BLACK, font_size),
                width - intend_x as Scalar,
                10,
            );
            let container_shape: (Scalar, Scalar) = message_container.get_shape(message, glyph_cache).unwrap();
            stack_pointer_y -= container_shape.0;
            if stack_pointer_y - intend_y <= rec_y {
                break;
            }
            message_container.draw(
                message,
                glyph_cache,
                &c.draw_state,
                c.transform.trans_pos([stack_pointer_x, stack_pointer_y]),
                gl,
            ).unwrap();
        }
    }
    pub(crate) fn add_message(&mut self, message: String) {
        self.messages.push(message)
    }
}
