use std::cell::RefCell;
use std::rc::Rc;

use graphics::{CharacterCache, Context, Graphics, Text, Transformed};
use graphics::math::{Matrix2d, Scalar};

use crate::constants::TEXT_FONT_SIZE;

pub struct Score {
    value: Rc<RefCell<u64>>,
}

impl Score {
    pub fn new() -> Self {
        Score {
            value: Rc::new(RefCell::new(0)),
        }
    }
    pub(crate) fn draw_pos<C, G>(&mut self, pos_x: Scalar, pos_y: Scalar, c: Context, gl: &mut G, transform: Matrix2d, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        self.draw(c, gl, transform.trans_pos([pos_x, pos_y]), glyph_cache)
    }
    pub(crate) fn draw<C, G>(&mut self, c: Context, gl: &mut G, transform: Matrix2d, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        Text::new(TEXT_FONT_SIZE).draw(
            self.value.borrow().to_string().as_str(),
            glyph_cache,
            &c.draw_state,
            transform,
            gl,
        ).unwrap();
    }

    pub(crate) fn inc(&self, value: u64) {
        *self.value.borrow_mut() += value;
    }
    pub(crate) fn get(&self) -> u64 {
        *self.value.borrow()
    }
}