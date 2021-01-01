use crate::screens::game_screen::colors::{UiColors};
use crate::screens::game_screen::config::SQUARE_WIDTH;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context};

pub fn draw_line(ctx: &mut Context, x1: u16, y1: u16, x2: u16, y2: u16, color: Color) {
    let (origin, destination) = (
        Point2::new(x1 as f32, y1 as f32),
        Point2::new(x2 as f32, y2 as f32),
    );

    let line = Mesh::new_line(ctx, &[origin, destination], 1.0, color).unwrap();
    graphics::draw(ctx, &line, DrawParam::new()).unwrap();
}

pub fn draw_line_f32(ctx: &mut Context, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    let (origin, destination) = (Point2::new(x1, y1), Point2::new(x2, y2));

    let line = Mesh::new_line(ctx, &[origin, destination], 1.0, color).unwrap();
    graphics::draw(ctx, &line, DrawParam::new()).unwrap();
}

pub fn draw_guide(ctx: &mut Context, x: u16, y: u16, width: u16, height: u16) {
    let rows = (height as f32 / SQUARE_WIDTH as f32) as u16;
    let columns = (width as f32 / SQUARE_WIDTH as f32) as u16;

    for i in 0..(rows + 1) {
        let line_y = y + i * SQUARE_WIDTH;
        draw_line_f32(
            ctx,
            x as f32,
            line_y as f32 + 0.5,
            x as f32 + width as f32,
            line_y as f32 + 0.5,
            UiColors::Guide.value(),
        );
    }
    for i in 0..(columns + 1) {
        let line_x = x + i * SQUARE_WIDTH;
        draw_line_f32(
            ctx,
            line_x as f32 + 0.5,
            y as f32,
            line_x as f32 + 0.5,
            y as f32 + height as f32,
            UiColors::Guide.value(),
        );
    }
}
