use crate::framework::graphics_painter;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::colors::ui_colors;
use crate::screens::game_screen::config::SQUARE_WIDTH;
use crate::screens::game_screen::shape::Shape;
use ggez::graphics::{Color, DrawParam};
use ggez::{graphics, Context};

pub fn clear(ctx: &mut Context) {
    graphics::clear(ctx, Color::from(ui_colors::BACKGROUND));
}

pub fn draw_shape(ctx: &mut Context, shape: Shape) {
    for square in shape.grid.iter() {
        draw_square_at(
            ctx,
            shape.row + square.row,
            shape.column + square.column,
            square.color,
        );
    }
}

pub fn draw_square_at(ctx: &mut Context, row: u16, column: u16, color: Color) {
    draw_tetromino_square(ctx, column * SQUARE_WIDTH, row * SQUARE_WIDTH, color)
}

pub fn draw_tetromino_square(ctx: &mut Context, x: u16, y: u16, color: Color) {
    let rect = graphics::Rect::new(x as f32, y as f32, SQUARE_WIDTH as f32, SQUARE_WIDTH as f32);
    let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color).unwrap();
    graphics::draw(ctx, &r1, DrawParam::default()).unwrap();
}

pub fn draw_guide(ctx: &mut Context) {
    graphics_painter::draw_guide(ctx, 0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
}
