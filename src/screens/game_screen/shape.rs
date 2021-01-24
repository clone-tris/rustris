use crate::screens::game_screen::config::{PUZZLE_HEIGHT, PUZZLE_WIDTH};
use crate::screens::game_screen::square::Square;
use ggez::graphics::Color;

#[derive(Debug, Clone)]
pub struct Shape {
    pub grid: Vec<Square>,
    pub row: i8,
    pub column: i8,
    pub width: i16,
    pub height: i16,
    color: Color,
}

impl Shape {
    pub fn new(grid: Vec<Square>, row: i8, column: i8, color: Color) -> Shape {
        let grid_size = grid.len();

        let mut shape = Shape {
            grid: grid,
            row,
            column,
            width: 0,
            height: 0,
            color,
        };

        if grid_size > 0 {
            shape.compute_size();
        }

        shape
    }

    pub fn compute_size(&mut self) {
        let mut min_row: i16 = PUZZLE_HEIGHT;
        let mut max_row: i16 = 0;
        let mut min_column: i16 = PUZZLE_WIDTH;
        let mut max_column: i16 = 0;

        self.grid.iter().for_each(|cell| {
            if cell.row > max_row {
                max_row = cell.row;
            }
            if cell.column > max_column {
                max_column = cell.column;
            }
            if cell.row < min_row {
                min_row = cell.row;
            }
            if cell.column < min_column {
                min_column = cell.column;
            }
        });

        self.height = max_row - min_row + 1;
        self.width = max_column - min_column + 1;
    }
}
