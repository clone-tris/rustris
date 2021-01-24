use crate::screens::game_screen::colors::ShapeColors;
use ggez::graphics::Color;

#[derive(Debug, Clone)]
pub struct Square {
    pub row: i16,
    pub column: i16,
    pub color: Color,
}

impl Square {
    pub fn new(row: i16, column: i16, color: Color) -> Square {
        Square { row, column, color }
    }
}
