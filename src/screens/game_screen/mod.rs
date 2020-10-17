pub mod colors;
pub mod config;
pub mod painter;
pub mod shape;
pub mod square;

use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context};

use crate::framework::graphics_painter::draw_line;
use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::shape_colors;
use crate::screens::game_screen::colors::shape_colors::CYAN;
use crate::screens::game_screen::config::SQUARE_WIDTH;
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
        painter::clear(ctx);
        painter::draw_guide(ctx);
        painter::draw_tetromino_square(ctx, SQUARE_WIDTH * 5, SQUARE_WIDTH * 4, Color::from(CYAN));
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode) {
        match key {
            KeyCode::Q => self.goto_over_screen = true,
            _ => (),
        }
    }

    fn canvas(&self) -> &Canvas {
        &self.canvas
    }
}
