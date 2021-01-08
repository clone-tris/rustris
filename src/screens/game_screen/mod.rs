pub mod colors;
pub mod config;
pub mod painter;
pub mod playfield;
pub mod shape;
pub mod sidebar;
pub mod square;

use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context};

use crate::framework::screen::Screen;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::config::WAR_ZONE_WIDTH;
use crate::screens::game_screen::playfield::PlayFieldScreen;
use crate::screens::over_screen::OverScreen;
use ggez::conf::NumSamples;

pub struct GameScreen {
    canvas: Canvas,
    playfield: PlayFieldScreen,
    goto_over_screen: bool,
}

impl GameScreen {
    pub fn new(ctx: &mut Context) -> GameScreen {
        GameScreen {
            canvas: graphics::Canvas::new(ctx, CANVAS_WIDTH, CANVAS_HEIGHT, NumSamples::One)
                .unwrap(),
            playfield: PlayFieldScreen::new(ctx, WAR_ZONE_WIDTH, CANVAS_HEIGHT),
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
        self.playfield.paint(ctx);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode) {
        match key {
            KeyCode::Q => self.goto_over_screen = true,
            _ => (),
        }
    }

    fn canvas(&self, ctx: &mut Context) -> &Canvas {
        let playfield_canvas = &self.playfield.canvas(ctx);
        painter::stitch(ctx, &self.canvas, playfield_canvas);
        &self.canvas
    }
}
