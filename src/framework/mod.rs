use ggez;
use nalgebra;

use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawParam};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer, Context, GameResult};
use ggez::{nalgebra as na, GameError};

trait Scene {
    fn update(&self, ctx: &mut Context);
    fn paint(&self, ctx: &mut Context);
    fn get_canvas(&self) -> &Canvas;
}

struct MenuScene {
    canvas: graphics::Canvas,
}

impl MenuScene {
    fn new(ctx: &mut Context) -> GameResult<MenuScene> {
        Ok(MenuScene {
            canvas: graphics::Canvas::with_window_size(ctx)?,
        })
    }
}

impl Scene for MenuScene {
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

pub struct Game {
    pub scene: Box<dyn Scene>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let scene = Box::new(MenuScene::new(ctx)?);
        let game = Game { scene };
        Ok(game)
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.scene.update(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = self.scene.get_canvas();

        graphics::set_canvas(ctx, Some(canvas));
        self.scene.paint(ctx);

        graphics::set_canvas(ctx, None);
        graphics::draw(ctx, canvas, DrawParam::new());

        graphics::present(ctx)?;
        Ok(())
    }
}
