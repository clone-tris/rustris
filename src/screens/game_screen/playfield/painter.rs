use crate::framework::graphics_painter;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::colors::{UiColors};
use crate::screens::game_screen::config::SQUARE_WIDTH;
use crate::screens::game_screen::shape::Shape;
use ggez::graphics::{Color, DrawParam};
use ggez::{graphics, Context};

pub struct Painter {
    width: u16,
    height: u16,
}

impl Painter {
    pub fn new(width: u16, height: u16) -> Painter {
        Painter { width, height }
    }

    pub fn clear(&self, ctx: &mut Context) {
        graphics::clear(ctx, UiColors::Background.value());
    }

    pub fn draw_shape(&self, ctx: &mut Context, shape: Shape) {
        for square in shape.grid.iter() {
            self.draw_square_at(
                ctx,
                shape.row + square.row,
                shape.column + square.column,
                square.color,
            );
        }
    }

    pub fn draw_square_at(&self, ctx: &mut Context, row: u16, column: u16, color: Color) {
        self.draw_tetromino_square(ctx, column * SQUARE_WIDTH, row * SQUARE_WIDTH, color)
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
}
