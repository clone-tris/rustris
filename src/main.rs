mod framework;
mod game_config;
mod screens;
use ggez::event;
use ggez::GameResult;
use ggez::{self, conf};
use std::env;
use std::path;

use crate::framework::game::Game;
use crate::game_config::CONFIG;
use crate::screens::menu_screen::MenuScreen;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")
    {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ref mut ctx, ref mut event_loop) =
        ggez::ContextBuilder::new(CONFIG.game_id, CONFIG.author)
            .window_setup(
                conf::WindowSetup::default().title(CONFIG.window_title),
            )
            .window_mode(
                conf::WindowMode::default()
                    .dimensions(CONFIG.width as f32, CONFIG.height as f32),
            )
            .add_resource_path(resource_dir)
            .build()
            .unwrap();

    let game = &mut Game::new(Box::new(MenuScreen::new(ctx)));

    event::run(ctx, event_loop, game)
}
