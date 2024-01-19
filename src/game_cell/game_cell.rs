use graphics::{CharacterCache, Context, Graphics, Text};
use graphics::color::RED;
use graphics::types::Scalar;

#[derive(Clone, Debug)]
pub struct Cell2048 {
    value: String,
}

impl Cell2048 {
    fn new(value: &String) -> Self {
        Cell2048 {
            value: String::from(value)
        }
    }
    fn is_empty(&self) -> bool {
        self.value == ""
    }
    fn draw<C, G>(&self, row: u32, col: u32, c: Context, gl: &mut G, glyph_cache: &mut C)
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let cell_center_x: Scalar = ((col + 1) * 100 - 100 / 2) as Scalar;
        let cell_center_y: Scalar = ((row + 1) * 100 - 100 / 2) as Scalar;
        Text::new_color(RED, 32).draw_pos(
            &*self.value,
            [cell_center_x, cell_center_y],
            glyph_cache,
            &c.draw_state,
            c.transform,
            gl,
        )
            .unwrap();
    }
}