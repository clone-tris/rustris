use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context};

use crate::framework::screen::Screen;
use crate::screens::menu_screen::MenuScreen;

pub struct OverScreen {
    canvas: Canvas,
    goto_menu_screen: bool,
}

impl OverScreen {
    pub fn new(ctx: &mut Context) -> OverScreen {
        OverScreen {
            canvas: graphics::Canvas::with_window_size(ctx).unwrap(),
            goto_menu_screen: false,
        }
    }
}

impl Screen for OverScreen {
    fn update(&mut self, ctx: &mut Context) -> Option<Box<dyn Screen>> {
        if self.goto_menu_screen {
            Some(Box::new(MenuScreen::new(ctx)))
        } else {
            None
        }
    }
    fn paint(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, Color::from((150, 200, 80, 128)));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match key {
            KeyCode::M => self.goto_menu_screen = true,
            _ => (),
        }
    }

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}
