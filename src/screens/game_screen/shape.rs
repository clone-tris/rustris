use crate::screens::game_screen::square::Square;
use crate::screens::game_screen::tetromino::ShapeGrid;
use ggez::graphics::Color;

#[derive(Debug, Clone)]
pub struct Shape {
    pub grid: Vec<Square>,
    pub row: u8,
    pub column: u8,
    color: Color,
}

impl Shape {
    pub fn new(shape_grid: ShapeGrid, row: u8, column: u8, color: Color) -> Shape {
        let grid = shape_grid
            .iter()
            .map(|coords| Square::new(coords[0], coords[1], color))
            .collect::<Vec<_>>();

        Shape {
            grid: grid.clone(),
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
