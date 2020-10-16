use crate::screens::game_screen::colors::shape_colors::{BLUE, CYAN, RED};
use crate::screens::game_screen::colors::ui_colors::GUIDE;
use crate::screens::game_screen::config::SQUARE_WIDTH;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub fn draw_line(ctx: &mut Context, x1: u16, y1: u16, x2: u16, y2: u16, color: Color) {
    let (origin, destination) = (
        Point2::new(x1 as f32, y1 as f32),
        Point2::new(x2 as f32, y2 as f32),
    );

    let line = Mesh::new_line(ctx, &[origin, destination], 1.0, color).unwrap();
    graphics::draw(ctx, &line, DrawParam::new()).unwrap();
}

pub fn draw_guide(ctx: &mut Context, x: u16, y: u16, width: u16, height: u16) {
    let puzzle_height = (height as f32 / SQUARE_WIDTH as f32) as u16;
    let puzzle_width = (width as f32 / SQUARE_WIDTH as f32) as u16;

    for i in 0..(puzzle_height + 1) {
        let y = x + i * SQUARE_WIDTH;
        draw_line(ctx, x, y, x + width, y, Color::from(GUIDE));
    }
    for i in 0..(puzzle_width + 1) {
        let x = y + i * SQUARE_WIDTH;
        draw_line(ctx, x, y, x, y + height, Color::from(GUIDE));
    }
}
