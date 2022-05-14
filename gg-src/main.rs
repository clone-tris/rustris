mod framework;
mod rustris_config;
mod screens;
mod test;

use ggez::event;
use ggez::GameResult;
use ggez::{self, conf};
use std::env;
use std::path;

use crate::framework::game::Game;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH, GAME_AUTHOR, GAME_ID, WINDOW_TITLE};
use crate::screens::game_screen::config::WAR_ZONE_WIDTH;
use crate::screens::game_screen::GameScreen;
use crate::test::Test;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let context_builder = ggez::ContextBuilder::new(GAME_ID, GAME_AUTHOR)
        .window_setup(conf::WindowSetup::default().title(WINDOW_TITLE))
        .window_mode(
            conf::WindowMode::default().dimensions(CANVAS_WIDTH as f32, CANVAS_HEIGHT as f32),
        )
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = context_builder.build()?;

    let game = Game::new(Box::new(GameScreen::new(&mut ctx)))?;
    let test = Test::new(&mut ctx)?;

    event::run(ctx, event_loop, game)
}
