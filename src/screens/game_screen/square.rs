use crate::screens::game_screen::colors::ShapeColors;
use ggez::graphics::Color;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub row: u8,
    pub column: u8,
    pub color: Color,
}

impl Square {
    pub fn new(row: u8, column: u8, color: Color) -> Square {
        Square { row, column, color }
    }

    pub fn default(row: u8, column: u8) -> Self {
        Square {
            row,
            column,
            color: ShapeColors::DefaultSquareColor.value(),
        }
    }
}
