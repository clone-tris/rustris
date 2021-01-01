use crate::screens::game_screen::config::SIDEBAR_WIDTH;
use ggez::graphics::{Canvas, DrawParam};
use ggez::{graphics, Context, GameResult};
use nalgebra::Point2;

pub fn stitch(ctx: &mut Context, game_canvas: &Canvas, playfield_canvas: &Canvas) {
    graphics::set_canvas(ctx, Some(game_canvas));
    graphics::draw(
        ctx,
        playfield_canvas,
        DrawParam::new().dest(Point2::new(SIDEBAR_WIDTH as f32, 0.0)),
    )
    .unwrap();
}
