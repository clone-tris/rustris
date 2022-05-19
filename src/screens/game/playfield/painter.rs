use crate::colors::UiColors;
use crate::engine::game_painter;
use crate::main_config::{CANVAS_HEIGHT, SQUARE_BORDER_WIDTH, SQUARE_WIDTH, WAR_ZONE_WIDTH};
use crate::screens::game::components::shape::Shape;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Painter {
    width: i32,
    height: i32,
}

impl Painter {
    pub fn new(width: i32, height: i32) -> Painter {
        Painter { width, height }
    }

    pub fn setup(&self, canvas: &mut WindowCanvas) {
        canvas.set_viewport(Rect::new(0, 0, WAR_ZONE_WIDTH as u32, CANVAS_HEIGHT as u32));
    }

    pub fn clear(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(UiColors::Background.value());
        canvas.clear();
    }

    pub fn draw_shape(&self, canvas: &mut WindowCanvas, shape: &Shape) {
        for square in shape.grid.iter() {
            self.draw_square_at(
                canvas,
                shape.row + square.row,
                shape.column + square.column,
                square.color,
            );
        }
    }

    pub fn draw_square_at(&self, canvas: &mut WindowCanvas, row: i32, column: i32, color: Color) {
        self.draw_tetromino_square(canvas, column * SQUARE_WIDTH, row * SQUARE_WIDTH, color);
    }

    pub fn draw_tetromino_square(&self, canvas: &mut WindowCanvas, x: i32, y: i32, color: Color) {
        let square_width = SQUARE_WIDTH as f32;
        let square_border_width = SQUARE_BORDER_WIDTH as f32;

        // Background
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x, y, SQUARE_WIDTH as u32, SQUARE_WIDTH as u32));

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

    pub fn draw_guide(&self, canvas: &mut WindowCanvas) {
        game_painter::draw_guide(canvas, 0, 0, self.width, self.height);
    }
}
