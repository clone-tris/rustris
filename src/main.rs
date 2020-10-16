mod framework;
mod screens;

use ggez::event;
use ggez::GameResult;
use ggez::{self, conf};
use std::env;
use std::path;

use crate::framework::game::Game;
use crate::screens::menu_screen::MenuScreen;

const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 460.0;

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")
    {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("rustris", "Abderrahmane Tahri Jouti")
        .window_setup(
            conf::WindowSetup::default()
                .title("Clonetris, made easy, in Rust!"),
        )
        .window_mode(
            conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT),
        )
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;

    let game = &mut Game::new(Box::new(MenuScreen::new(ctx)));

    event::run(ctx, event_loop, game)
}
