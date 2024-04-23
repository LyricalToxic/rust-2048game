use graphics::{CharacterCache, DrawState, Graphics, Image, Text, Transformed};
use graphics::math::{Matrix2d, Scalar};

pub struct TextContainer {
    text: Text,
    width: Scalar,
    max_lines: usize,
    actual_lines: usize,
}

impl TextContainer {
    pub(crate) fn new(text: Text, width: Scalar, max_lines: usize) -> Self {
        TextContainer {
            text,
            width,
            max_lines,
            actual_lines: 0,
        }
    }

    pub fn draw<C, G>(
        &self,
        text: &str,
        cache: &mut C,
        draw_state: &DrawState,
        transform: Matrix2d,
        g: &mut G,
    ) -> Result<(), C::Error>
        where
            C: CharacterCache,
            G: Graphics<Texture=<C as CharacterCache>::Texture>,
    {
        let mut image = Image::new_color(self.text.color);

        let mut x = 0.0;
        let mut y = 0.0;
        let line_spacing = 10.0;
        let mut line_width: Scalar = 0.0;
        for ch in text.chars() {
            let character = cache.character(self.text.font_size, ch)?;
            line_width += character.advance_width();
            if line_width >= self.width {
                line_width = character.advance_width();
                y += character.atlas_size[1] + line_spacing;
                x = 0.0;
            }

            let ch_x = x + character.left();
            let ch_y = y - character.top();
            image = image.src_rect([
                character.atlas_offset[0],
                character.atlas_offset[1],
                character.atlas_size[0],
                character.atlas_size[1],
            ]);
            image.draw(
                character.texture,
                draw_state,
                transform.trans(ch_x, ch_y),
                g,
            );
            x += character.advance_width();
            y += character.advance_height();
        }
        Ok(())
    }
    pub fn get_shape<C>(&self, text: &str, cache: &mut C) -> Result<((Scalar, Scalar)), C::Error>
        where
            C: CharacterCache
    {
        let mut x = 0.0;
        let mut y = 0.0;
        let line_spacing: Scalar = 10.0;
        let mut line_width: Scalar = 0.0;
        let mut last_ch_height: Scalar = 0.0;
        for ch in text.chars() {
            let character = cache.character(self.text.font_size, ch)?;
            line_width += character.advance_width();
            if line_width >= self.width {
                line_width = character.advance_width();
                y += character.atlas_size[1] + line_spacing;
                x = 0.0;
            }

            x += character.advance_width();
            y += character.advance_height();
            last_ch_height = character.atlas_size[1];
        }
        Ok((y + last_ch_height + line_spacing, self.width))
    }
}