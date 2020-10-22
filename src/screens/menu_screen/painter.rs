use crate::framework::graphics_painter;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::colors::{ui_color, UiColors};
use ggez::{graphics, Context};

pub fn draw_background(ctx: &mut Context) {
    clear(ctx);
    draw_guide(ctx);
}

pub fn clear(ctx: &mut Context) {
    graphics::clear(ctx, ui_color(UiColors::Background));
}

pub fn draw_guide(ctx: &mut Context) {
    graphics_painter::draw_guide(ctx, 0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
}
