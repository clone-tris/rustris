use crate::main_config::SQUARE_WIDTH;
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
        let y = (self.row + reference_row) * SQUARE_WIDTH;
        let x = (self.column + reference_column) * SQUARE_WIDTH;

        // // Background
        canvas.set_draw_color(self.color);
        canvas
            .fill_rect(Rect::new(x, y, SQUARE_WIDTH as u32, SQUARE_WIDTH as u32))
            .unwrap();

        // Left Border
        // mb.polygon(
        //     DrawMode::fill(),
        //     &[
        //         glam::Vec2::new(x, y),
        //         glam::Vec2::new(x + square_border_width, y + square_border_width),
        //         glam::Vec2::new(
        //             x + square_border_width,
        //             y + square_width - square_border_width,
        //         ),
        //         glam::Vec2::new(x, y + square_width),
        //     ],
        //     ShapeColors::BorderSide.value(),
        // )
        // .unwrap();

        // Right Border
        // mb.polygon(
        //     DrawMode::fill(),
        //     &[
        //         glam::Vec2::new(x + square_width, y),
        //         glam::Vec2::new(
        //             x + square_width - square_border_width,
        //             y + square_border_width,
        //         ),
        //         glam::Vec2::new(
        //             x + square_width - square_border_width,
        //             y + square_width - square_border_width,
        //         ),
        //         glam::Vec2::new(x + square_width, y + square_width),
        //     ],
        //     ShapeColors::BorderSide.value(),
        // )
        // .unwrap();

        // Top Border
        // mb.polygon(
        //     DrawMode::fill(),
        //     &[
        //         glam::Vec2::new(x, y),
        //         glam::Vec2::new(x + square_border_width, y + square_border_width),
        //         glam::Vec2::new(
        //             x + square_width - square_border_width,
        //             y + square_border_width,
        //         ),
        //         glam::Vec2::new(x + square_width, y),
        //     ],
        //     ShapeColors::BorderTop.value(),
        // )
        // .unwrap();

        // Bottom Border
        // mb.polygon(
        //     DrawMode::fill(),
        //     &[
        //         glam::Vec2::new(x, y + square_width),
        //         glam::Vec2::new(
        //             x + square_border_width,
        //             y + square_width - square_border_width,
        //         ),
        //         glam::Vec2::new(
        //             x + square_width - square_border_width,
        //             y + square_width - square_border_width,
        //         ),
        //         glam::Vec2::new(x + square_width, y + square_width),
        //     ],
        //     ShapeColors::BorderBottom.value(),
        // )
        // .unwrap();

        // let mesh = mb.build(ctx).unwrap();
        // graphics::draw(ctx, &mesh, DrawParam::default()).unwrap();
    }
}
