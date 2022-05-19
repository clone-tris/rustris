use crate::screens::game::components::colors::ShapeColors;
use crate::screens::game::components::shape::Shape;
use crate::screens::game::components::square::Square;
use rand::seq::SliceRandom;
use sdl2::pixels::Color;

#[derive(Debug, Clone, Copy)]
pub enum Tetromino {
    T,
    Z,
    S,
    L,
    J,
    O,
    I,
}

pub type ShapeGrid = [[i16; 2]; 4];

pub fn tetromino_grid(tetromino: &Tetromino) -> ShapeGrid {
    return match tetromino {
        Tetromino::T => [[0, 0], [0, 1], [0, 2], [1, 1]],
        Tetromino::Z => [[0, 0], [0, 1], [1, 1], [1, 2]],
        Tetromino::S => [[0, 1], [0, 2], [1, 0], [1, 1]],
        Tetromino::L => [[0, 0], [0, 1], [0, 2], [1, 0]],
        Tetromino::J => [[0, 0], [1, 0], [1, 1], [1, 2]],
        Tetromino::O => [[0, 0], [0, 1], [1, 0], [1, 1]],
        Tetromino::I => [[0, 0], [0, 1], [0, 2], [0, 3]],
    };
}

pub fn tetromino_color(tetromino: &Tetromino) -> Color {
    return match tetromino {
        Tetromino::T => ShapeColors::Purple.value(),
        Tetromino::Z => ShapeColors::Red.value(),
        Tetromino::S => ShapeColors::Green.value(),
        Tetromino::L => ShapeColors::Orange.value(),
        Tetromino::J => ShapeColors::Blue.value(),
        Tetromino::O => ShapeColors::Yellow.value(),
        Tetromino::I => ShapeColors::Cyan.value(),
    };
}

pub fn random_tetromino() -> Shape {
    let mut rng = rand::thread_rng();
    let tetromino = [
        Tetromino::T,
        Tetromino::Z,
        Tetromino::S,
        Tetromino::L,
        Tetromino::J,
        Tetromino::O,
        Tetromino::I,
    ]
    .choose(&mut rng)
    .unwrap();

    let grid = tetromino_grid(tetromino);
    let color = tetromino_color(tetromino);

    let square_grid = grid
        .iter()
        .map(|coords| Square::new(coords[0], coords[1], color))
        .collect::<Vec<_>>();

    return Shape::new(square_grid, 0, 0, color);
}
