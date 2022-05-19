use sdl2::pixels::Color;

#[derive(Debug, Clone)]
pub struct Square {
    pub row: i16,
    pub column: i16,
    pub color: sdl2::pixels::Color,
}

impl Square {
    pub fn new(row: i16, column: i16, color: Color) -> Square {
        Square { row, column, color }
    }
}
