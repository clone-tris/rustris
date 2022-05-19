use sdl2::pixels::Color;

#[derive(Debug, Clone)]
pub struct Square {
    pub row: i32,
    pub column: i32,
    pub color: Color,
}

impl Square {
    pub fn new(row: i32, column: i32, color: Color) -> Square {
        Square { row, column, color }
    }
}
