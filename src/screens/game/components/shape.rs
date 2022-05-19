use crate::main_config::{PUZZLE_HEIGHT, PUZZLE_WIDTH};
use crate::screens::game::components::square::Square;
use sdl2::pixels::Color;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Shape {
    pub grid: Vec<Square>,
    pub row: i32,
    pub column: i32,
    pub width: i32,
    pub height: i32,
    color: Color,
}

impl Shape {
    pub fn new(grid: Vec<Square>, row: i32, column: i32, color: Color) -> Shape {
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
        let mut min_row: i32 = PUZZLE_HEIGHT as i32;
        let mut max_row: i32 = 0;
        let mut min_column: i32 = PUZZLE_WIDTH as i32;
        let mut max_column: i32 = 0;

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

    pub fn translate(&mut self, row_direction: u8, column_direction: i32) {
        self.row += row_direction as i32;
        self.column += column_direction;
    }

    pub fn absolute_grid(&self) -> Vec<Square> {
        self.grid
            .iter()
            .map(|square| {
                let mut absolute_square = square.clone();
                absolute_square.row = square.row + self.row as i32;
                absolute_square.column = square.column + self.column as i32;
                absolute_square
            })
            .collect()
    }

    pub fn collides_with(&self, b: &Shape) -> bool {
        self.absolute_grid().iter().any(|square_a| {
            b.absolute_grid()
                .iter()
                .any(|square_b| square_b.row == square_a.row && square_b.column == square_a.column)
        })
    }

    pub fn merge(&mut self, b: Shape) {
        let mut a_grid = self.absolute_grid();
        let mut b_grid = b.absolute_grid();
        a_grid.append(&mut b_grid);
        self.grid = a_grid;
    }

    pub fn within_bounds(&self) -> bool {
        let absolute_grid = self.absolute_grid();

        let after_right = absolute_grid
            .iter()
            .any(|square| square.column >= PUZZLE_WIDTH as i32);
        if after_right {
            return false;
        }

        let before_left = absolute_grid.iter().any(|square| square.column < 0);
        if before_left {
            return false;
        }

        let bellow_bottom = absolute_grid
            .iter()
            .any(|square| square.row >= PUZZLE_HEIGHT as i32);
        if bellow_bottom {
            return false;
        }

        true
    }

    pub fn rotate(&mut self) {
        self.grid = self
            .grid
            .iter()
            .map(|square| {
                let mut new_square = square.clone();
                new_square.row = square.column;
                new_square.column = self.height - square.row - 1;
                new_square
            })
            .collect();
        self.compute_size();
    }

    pub fn remove_full_lines(&mut self) -> i32 {
        let full_rows = self.find_full_rows();
        if full_rows.len() == 0 {
            return 0;
        }

        let mut new_grid: Vec<Square> = Vec::new();

        for square in &self.grid {
            if full_rows.contains(&square.row) {
                continue;
            }

            let mut clone = square.clone();

            let mut square_row_before_shifting = clone.row;

            for full_row in &full_rows {
                if full_row > &square_row_before_shifting {
                    clone.row += 1;
                }
            }

            new_grid.push(clone);
        }

        self.grid = new_grid;

        return full_rows.len() as i32;
    }

    // todo : only look for full rows in the height of the player that was just eaten
    pub fn find_full_rows(&self) -> HashSet<i32> {
        let mut full_rows = HashSet::<i32>::new();
        let mut row_population = HashMap::<i32, i32>::new();

        for square in self.grid.iter() {
            *row_population.entry(square.row).or_insert(0) += 1;
            if row_population.get(&square.row) == Some(&(PUZZLE_WIDTH as i32)) {
                full_rows.insert(square.row);
            }
        }

        full_rows
    }
}
