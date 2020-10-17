use crate::screens::game_screen::square::Square;
use ggez::graphics::Color;

#[derive(Debug, Clone)]
pub struct Shape {
    grid: Vec<Square>,
    row: u16,
    column: u16,
    color: Color,
    width: u16,
    height: u16,
    compute_height: bool,
}

impl Shape {
    pub fn new(
        grid: Vec<Square>,
        row: u16,
        column: u16,
        color: Color,
        width: u16,
        height: u16,
        compute_height: bool,
    ) -> Shape {
        Shape {
            grid,
            row,
            column,
            color,
            width,
            height,
            compute_height,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        for square in self.grid.iter_mut() {
            square.color = color;
        }
    }
}
