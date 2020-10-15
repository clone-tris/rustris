use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::{graphics, Context, GameResult};

use crate::framework::screen::Screen;

pub struct Game {
    screen: Box<dyn Screen>,
}

impl Game {
    pub fn new(initial_screen: Box<dyn Screen>) -> GameResult<Game> {
        let game = Game {
            screen: initial_screen,
        };
        Ok(game)
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.update(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.screen.paint(ctx);
        let canvas = self.screen.canvas();
        graphics::set_canvas(ctx, None);
        graphics::draw(ctx, canvas, DrawParam::new())?;
        graphics::present(ctx)?;
        Ok(())
    }
}
