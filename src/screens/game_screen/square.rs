use ggez::graphics::Color;

#[derive(Debug, Clone, Copy)]
pub struct Square {
    row: u16,
    column: u16,
    pub color: Color,
}

impl Square {
    pub fn new(row: u16, column: u16, color: Color) -> Square {
        Square { row, column, color }
    }
}
