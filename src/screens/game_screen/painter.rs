use crate::rustris_config::CANVAS_HEIGHT;
use crate::screens::game_screen::config::{SIDEBAR_WIDTH, WAR_ZONE_WIDTH};
use ggez::graphics::{Canvas, DrawParam};
use ggez::{graphics, Context, GameResult};
use nalgebra::Point2;

pub fn stitch(ctx: &mut Context, game_canvas: &Canvas, playfield_canvas: &Canvas) -> GameResult {
    graphics::set_canvas(ctx, Some(game_canvas));
    graphics::set_screen_coordinates(
        ctx,
        [
            0.0,
            0.0,
            (SIDEBAR_WIDTH + WAR_ZONE_WIDTH) as f32,
            CANVAS_HEIGHT as f32,
        ]
        .into(),
    )?;
    graphics::draw(
        ctx,
        playfield_canvas,
        DrawParam::new().dest(Point2::new(SIDEBAR_WIDTH as f32, 0.0)),
    )?;

    Ok(())
}
