pub mod colors;
pub mod config;
pub mod painter;
pub mod shape;
pub mod square;

use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context};

use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::{shaope_color, ShapeColors};
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::square::Square;
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

        let shape = Shape::new(
            vec![
                Square::default(0, 0),
                Square::default(1, 0),
                Square::default(1, 1),
                Square::default(1, 2),
            ],
            2,
            2,
            shaope_color(ShapeColors::Cyan),
        );

        painter::draw_shape(ctx, shape)
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
