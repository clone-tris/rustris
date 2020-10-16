use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context};

use crate::framework::graphics_painter;
use crate::framework::graphics_painter::draw_guide;
use crate::framework::screen::Screen;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::colors::shape_colors;
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
        graphics::clear(ctx, Color::from(shape_colors::ORANGE));
        graphics_painter::draw_line(ctx, 10, 30, 100, 300, Color::from_rgb(50, 100, 150));
        draw_guide(ctx, 0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match key {
            KeyCode::S => self.goto_game_screen = true,
            _ => (),
        }
    }

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}
