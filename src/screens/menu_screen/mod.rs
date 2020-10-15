use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::{graphics, timer, Context, GameResult};

use crate::framework::screen::Screen;

pub struct MenuScreen {
    canvas: Canvas,
}

impl MenuScreen {
    pub fn new(ctx: &mut Context) -> GameResult<MenuScreen> {
        Ok(MenuScreen {
            canvas: graphics::Canvas::with_window_size(ctx)?,
        })
    }
}

impl Screen for MenuScreen {
    fn update(&self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            let time = timer::time_since_start(ctx).as_secs_f32() as i32;
            println!("{}", time);
        }
    }
    fn paint(&self, ctx: &mut Context) {
        graphics::clear(ctx, Color::from((80, 150, 200, 128)));
    }

    fn get_canvas(&self) -> &Canvas {
        &self.canvas
    }
}
