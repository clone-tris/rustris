use crate::screens::game_screen::square::Square;
use ggez::graphics::Color;

#[derive(Debug, Clone)]
pub struct Shape {
    pub grid: Vec<Square>,
    pub row: u16,
    pub column: u16,
    color: Color,
}

impl Shape {
    pub fn new(grid: Vec<Square>, row: u16, column: u16, color: Color) -> Shape {
        Shape {
            grid,
            row,
            column,
            color,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        for square in self.grid.iter_mut() {
            square.color = color;
        }
    }
}
