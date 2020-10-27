use ggez::graphics::{Canvas, DrawParam};
use ggez::{graphics, Context, GameResult};

pub fn stitch(ctx: &mut Context, game_canvas: &Canvas, playfield_canvas: &Canvas) {
    graphics::set_canvas(ctx, Some(game_canvas));
    graphics::draw(ctx, playfield_canvas, DrawParam::new()).unwrap();
}
