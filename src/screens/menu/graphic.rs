use crate::colors::ShapeColors;
use crate::screens::game::components::square::Square;

const GRAPHIC_CORDS: [[i32; 2]; 43] = [
    [1, 1],
    [2, 1],
    [3, 1],
    [3, 2],
    [3, 4],
    [3, 5],
    [3, 6],
    [2, 6],
    [1, 6],
    [1, 5],
    [1, 4],
    [2, 4],
    [2, 8],
    [1, 9],
    [3, 9],
    [3, 10],
    [2, 10],
    [5, 3],
    [6, 3],
    [6, 2],
    [7, 3],
    [8, 2],
    [7, 1],
    [5, 5],
    [7, 5],
    [8, 5],
    [8, 7],
    [7, 7],
    [6, 7],
    [6, 8],
    [6, 9],
    [7, 9],
    [8, 9],
    [10, 2],
    [11, 1],
    [12, 2],
    [11, 3],
    [12, 3],
    [13, 3],
    [14, 2],
    [14, 5],
    [14, 6],
    [14, 7],
];

pub fn get_graphic() -> Vec<Square> {
    GRAPHIC_CORDS
        .iter()
        .map(|coords| {
            Square::new(
                coords[0],
                coords[1],
                ShapeColors::DefaultSquareColor.value(),
            )
        })
        .collect::<Vec<_>>()
}
