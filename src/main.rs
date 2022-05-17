extern crate sdl2;

mod framework;
mod screens;

use crate::framework::game_manager::GameManager;
use crate::screens::game::Game;
use crate::screens::menu::Menu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut manager = GameManager::new(Box::new(Menu::new()), &mut canvas, &mut event_pump);
    manager.gameloop();
}
