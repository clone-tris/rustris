use crate::colors::ShapeColors::{BorderBottom, BorderSide, BorderTop};
use crate::game_config::{SQUARE_BORDER_WIDTH, SQUARE_WIDTH};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub struct Square {
    pub row: i32,
    pub column: i32,
    pub color: Color,
}

impl Square {
    pub fn new(row: i32, column: i32, color: Color) -> Square {
        Square { row, column, color }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, reference_row: i32, reference_column: i32) {
        let y = ((self.row + reference_row) * SQUARE_WIDTH) as i16;
        let x = ((self.column + reference_column) * SQUARE_WIDTH) as i16;
        const BORDER_WIDTH: i16 = SQUARE_BORDER_WIDTH as i16;
        const WIDTH: i16 = SQUARE_WIDTH as i16;

        // // Background
        canvas.set_draw_color(self.color);
        canvas
            .fill_rect(Rect::new(
                x as i32,
                y as i32,
                SQUARE_WIDTH as u32,
                SQUARE_WIDTH as u32,
            ))
            .unwrap();

        // Left Border
        canvas
            .filled_polygon(
                &[x, x + BORDER_WIDTH, x + BORDER_WIDTH, x],
                &[y, y + BORDER_WIDTH, y + WIDTH - BORDER_WIDTH, y + WIDTH],
                BorderSide.value(),
            )
            .unwrap();

        // Right Border
        canvas
            .filled_polygon(
                &[
                    x + WIDTH,
                    x + WIDTH - BORDER_WIDTH,
                    x + WIDTH - BORDER_WIDTH,
                    x + WIDTH,
                ],
                &[y, y + BORDER_WIDTH, y + WIDTH - BORDER_WIDTH, y + WIDTH],
                BorderSide.value(),
            )
            .unwrap();

        // Top Border
        canvas
            .filled_polygon(
                &[x, x + BORDER_WIDTH, x + WIDTH - BORDER_WIDTH, x + WIDTH],
                &[y, y + BORDER_WIDTH, y + BORDER_WIDTH, y],
                BorderTop.value(),
            )
            .unwrap();

        // Bottom Border
        canvas
            .filled_polygon(
                &[x, x + BORDER_WIDTH, x + WIDTH - BORDER_WIDTH, x + WIDTH],
                &[
                    y + WIDTH,
                    y + WIDTH - BORDER_WIDTH,
                    y + WIDTH - BORDER_WIDTH,
                    y + WIDTH,
                ],
                BorderBottom.value(),
            )
            .unwrap();
    }
}
