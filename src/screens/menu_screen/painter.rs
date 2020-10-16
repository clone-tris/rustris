use crate::framework::graphics_painter;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::colors::ui_colors;
use ggez::graphics::Color;
use ggez::{graphics, Context};

pub fn clear(ctx: &mut Context) {
    graphics::clear(ctx, Color::from(ui_colors::BACKGROUND));
}

pub fn draw_background(ctx: &mut Context) {
    graphics_painter::draw_guide(ctx, 0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
}
