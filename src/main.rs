extern crate sdl2;

mod colors;
mod engine;
mod main_config;
mod screens;

use crate::engine::game_manager::GameManager;
use crate::main_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game::Game;
use crate::screens::menu::Menu;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut manager = GameManager::new(Box::new(Game::new()), &mut canvas, &mut event_pump);
    manager.gameloop();
}
