use graphics::{DrawState, Graphics, Line};
use graphics::math::{Matrix2d, Scalar};

pub struct PositionGrid {
    cols: usize,
    rows: usize,
    units: Scalar,
    offset: Scalar,
}

impl PositionGrid {
    pub fn new(cols: usize, rows: usize, units: Scalar, offset: Option<Scalar>) -> Self {
        PositionGrid {
            cols,
            rows,
            units,
            offset: offset.unwrap_or_else(|| 0.0),
        }
    }
    pub fn draw<G>(&self, line: &Line, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where
            G: Graphics,
    {
        for row in 0..self.rows + 1 {
            let x1 = row as Scalar * self.units + self.offset;
            let y1 = self.offset;
            let x2 = row as Scalar * self.units + self.offset;
            let y2 = self.rows as Scalar * self.units + self.offset;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
        for col in 0..self.cols + 1 {
            let x1 = self.offset;
            let y1 = col as Scalar * self.units + self.offset;
            let x2 = self.cols as Scalar * self.units + self.offset;
            let y2 = col as Scalar * self.units + self.offset;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
    }
    pub fn get_pos(&self, (row, col): (usize, usize)) -> (Scalar, Scalar) {
        (row as Scalar * self.units + self.offset, col as Scalar * self.units + self.offset)
    }
    pub fn get_rectangle_pos(&self) -> [Scalar; 4] {
        let pos_x1 = self.offset;
        let pos_y1 = self.offset;
        let pos_x2 = self.cols as Scalar * self.units + self.offset;
        let pos_y2 = self.rows as Scalar * self.units + self.offset;
        [pos_x1, pos_y1, pos_x2, pos_y2]
    }
}