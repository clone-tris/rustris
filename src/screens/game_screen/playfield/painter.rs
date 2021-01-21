use crate::framework::graphics_painter;
use crate::rustris_config::CANVAS_HEIGHT;
use crate::screens::game_screen::colors::{ShapeColors, UiColors};
use crate::screens::game_screen::config::{SQUARE_WIDTH, WAR_ZONE_WIDTH};
use crate::screens::game_screen::shape::Shape;
use ggez::graphics::{Canvas, Color, DrawParam, Mesh};
use ggez::{graphics, Context};
use nalgebra::Point2;

pub struct Painter {
    width: u16,
    height: u16,
}

impl Painter {
    pub fn new(width: u16, height: u16) -> Painter {
        Painter { width, height }
    }

    pub fn setup(&self, ctx: &mut Context, canvas: &Canvas) {
        graphics::set_canvas(ctx, Some(canvas));
        graphics::set_screen_coordinates(
            ctx,
            [0.0, 0.0, WAR_ZONE_WIDTH as f32, CANVAS_HEIGHT as f32].into(),
        )
        .unwrap();
    }

    pub fn clear(&self, ctx: &mut Context) {
        graphics::clear(ctx, UiColors::Background.value());
    }

    pub fn draw_shape(&self, ctx: &mut Context, shape: &Shape) {
        for square in shape.grid.iter() {
            self.draw_square_at(
                ctx,
                shape.row + square.row,
                shape.column + square.column,
                square.color,
            );
        }
    }

    pub fn draw_square_at(&self, ctx: &mut Context, row: u8, column: u8, color: Color) {
        self.draw_tetromino_square(
            ctx,
            column as u16 * SQUARE_WIDTH,
            row as u16 * SQUARE_WIDTH,
            color,
        )
    }

    pub fn draw_tetromino_square(&self, ctx: &mut Context, x: u16, y: u16, color: Color) {
        let rect =
            graphics::Rect::new(x as f32, y as f32, SQUARE_WIDTH as f32, SQUARE_WIDTH as f32);
        let r1 =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color).unwrap();
        graphics::draw(ctx, &r1, DrawParam::default()).unwrap();
    }

    pub fn draw_guide(&self, ctx: &mut Context) {
        graphics_painter::draw_guide(ctx, 0, 0, self.width, self.height);
    }

    pub fn draw_green_line(&self, ctx: &mut Context) {
        // graph\ics_painter::draw_line(ctx, 0, 10, self.width, 10, ShapeColors::Green.value());

        let line = Mesh::new_line(
            ctx,
            &[Point2::new(0.0, 20.0), Point2::new(self.width as f32, 20.0)],
            10.0,
            ShapeColors::Green.value(),
        )
        .unwrap();
        graphics::draw(ctx, &line, DrawParam::new()).unwrap();
    }
}
