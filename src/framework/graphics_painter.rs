use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub fn draw_line(ctx: &mut Context, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    let (origin, destination) = (
        Point2::new(x1 as f32, y1 as f32),
        Point2::new(x2 as f32, y2 as f32),
    );

    let line = Mesh::new_line(ctx, &[origin, destination], 1.0, color).unwrap();

    graphics::draw(ctx, &line, DrawParam::new()).unwrap();
}
