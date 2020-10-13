use ggez::event::EventHandler;
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, timer, Context, GameResult};

trait Scene {
    fn update(&self, ctx: &mut Context);
}

struct MenuScene {}

impl MenuScene {
    fn new() -> MenuScene {
        MenuScene {}
    }
}

impl Scene for MenuScene {
    fn update(&self, ctx: &mut Context) {
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            let time = timer::time_since_start(ctx).as_secs_f32() as i32;
            println!("{}", time);
        }
    }
}

pub struct Game {
    scene: Box<dyn Scene>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let scene = Box::new(MenuScene::new());
        let mut game = Game { scene };
        Ok(game)
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.scene.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)?;
        Ok(())
    }
}
