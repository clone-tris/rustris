mod painter;

use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context};

use crate::framework::screen::Screen;
use crate::screens::game_screen::GameScreen;

pub struct MenuScreen {
    canvas: Canvas,
    goto_game_screen: bool,
}

impl MenuScreen {
    pub fn new(ctx: &mut Context) -> MenuScreen {
        MenuScreen {
            canvas: graphics::Canvas::with_window_size(ctx).unwrap(),
            goto_game_screen: false,
        }
    }
}

impl Screen for MenuScreen {
    fn update(&mut self, ctx: &mut Context) -> Option<Box<dyn Screen>> {
        if self.goto_game_screen {
            Some(Box::new(GameScreen::new(ctx)))
        } else {
            None
        }
    }

    fn paint(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, Some(&self.canvas));
        painter::draw_background(ctx);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode) {
        match key {
            KeyCode::S => self.goto_game_screen = true,
            _ => (),
        }
    }

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}
