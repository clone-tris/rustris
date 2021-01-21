use crate::screens::game_screen::colors::ShapeColors;
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::square::Square;
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

pub fn random_tetromino() -> Shape {
    let mut rng = rand::thread_rng();
    let value = [
        Tetromino::T,
        Tetromino::Z,
        Tetromino::S,
        Tetromino::L,
        Tetromino::J,
        Tetromino::O,
        Tetromino::I,
    ]
    .choose(&mut rng);

    println!("{:?}", value);

    return Shape::new(
        vec![
            Square::default(0, 0),
            Square::default(1, 0),
            Square::default(1, 1),
            Square::default(1, 2),
        ],
        2,
        2,
        ShapeColors::Cyan.value(),
    );
}
