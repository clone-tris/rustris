use crate::screens::game_screen::colors::{ShapeColors};
use ggez::graphics::Color;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub row: u16,
    pub column: u16,
    pub color: Color,
}

impl Square {
    pub fn new(row: u16, column: u16, color: Color) -> Square {
        Square { row, column, color }
    }

    pub fn default(row: u16, column: u16) -> Self {
        Square {
            row,
            column,
            color: ShapeColors::DefaultSquareColor.value(),
        }
    }
}
