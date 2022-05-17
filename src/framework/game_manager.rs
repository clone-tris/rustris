use crate::framework::screen::Screen;
use crate::framework::screen_name::ScreenName;
use crate::{Game, Menu};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::borrow::{Borrow, BorrowMut};
use std::mem::transmute;
use std::time::Duration;

pub struct GameManager<'m> {
    screen: Box<dyn Screen + 'm>,
    canvas: &'m mut WindowCanvas,
    event_pump: &'m mut EventPump,
}

impl<'m> GameManager<'m> {
    pub(crate) fn new(
        screen: Box<dyn Screen + 'm>,
        canvas: &'m mut WindowCanvas,
        event_pump: &'m mut EventPump,
    ) -> GameManager<'m> {
        GameManager {
            canvas,
            screen,
            event_pump,
        }
    }

    pub(crate) fn gameloop(&mut self) {
        loop {
            for event in self.event_pump.poll_iter().collect::<Vec<_>>() {
                if self.handle_event(event.clone()) || self.screen.handle_event(event.clone()) {
                    return;
                }
            }
            self.update();
            self.paint();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub(crate) fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            _ => {}
        };

        return self.screen.handle_event(event);
    }

    pub(crate) fn update(&mut self) {
        if let Some(next_screen_name) = self.screen.update() {
            self.screen = self.spaw_screen(next_screen_name);
        }
    }

    fn spaw_screen(&mut self, screen: ScreenName) -> Box<dyn Screen> {
        match screen {
            ScreenName::Menu => Box::new(Menu::new()),
            ScreenName::Game => Box::new(Game::new()),
        }
    }

    pub(crate) fn paint(&mut self) {
        self.screen.paint(self.canvas);
        self.canvas.present();
    }
}
