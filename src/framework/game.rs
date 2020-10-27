use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::{graphics, Context, GameResult};

use crate::framework::screen::Screen;
use ggez::input::keyboard::{KeyCode, KeyMods};

pub struct Game {
    screen: Box<dyn Screen>,
}

impl Game {
    pub fn new(initial_screen: Box<dyn Screen>) -> Game {
        let game = Game {
            screen: initial_screen,
        };
        game
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(next_screen) = self.screen.update(ctx) {
            self.screen.unload(ctx);
            self.screen = next_screen;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.paint(ctx);
        let canvas = self.screen.canvas(ctx);
        graphics::set_canvas(ctx, None);
        graphics::draw(ctx, canvas, DrawParam::new())?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.screen.key_down_event(ctx, keycode);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.screen.key_up_event(ctx, keycode);
    }
}
