pub mod config;
pub mod colors;

use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context};

use crate::framework::screen::Screen;
use crate::screens::over_screen::OverScreen;

pub struct GameScreen {
    canvas: Canvas,
    goto_over_screen: bool,
}

impl GameScreen {
    pub fn new(ctx: &mut Context) -> GameScreen {
        GameScreen {
            canvas: graphics::Canvas::with_window_size(ctx).unwrap(),
            goto_over_screen: false,
        }
    }
}

impl Screen for GameScreen {
    fn update(&mut self, ctx: &mut Context) -> Option<Box<dyn Screen>> {
        if self.goto_over_screen {
            Some(Box::new(OverScreen::new(ctx)))
        } else {
            None
        }
    }

    fn paint(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, Color::from((200, 80, 150, 128)));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match key {
            KeyCode::Q => self.goto_over_screen = true,
            _ => (),
        }
    }

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}
