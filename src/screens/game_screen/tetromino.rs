use crate::screens::game_screen::colors::ShapeColors;
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::square::Square;
use ggez::graphics::Color;
use ggez::GameResult;
use rand::seq::SliceRandom;

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

pub type ShapeGrid = [[u8; 2]; 4];

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

    println!("{:?}", tetromino_color(tetromino));
    return Shape::new(tetromino_grid(tetromino), 2, 2, tetromino_color(tetromino));
}
