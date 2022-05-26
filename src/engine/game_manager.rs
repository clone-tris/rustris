use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use crate::screens::game::Game;
use crate::Menu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use std::time::Duration;

pub struct GameManager<'m> {
    screen: Box<dyn Screen + 'm>,
    canvas: &'m mut WindowCanvas,
    event_pump: &'m mut EventPump,
    font: &'m Font<'m, 'm>,
    texture_creator: &'m TextureCreator<WindowContext>,
    close_application: bool,
}

impl<'m> GameManager<'m> {
    pub(crate) fn new(
        screen: Box<dyn Screen + 'm>,
        canvas: &'m mut WindowCanvas,
        event_pump: &'m mut EventPump,
        texture_creator: &'m TextureCreator<WindowContext>,
        font: &'m Font<'m, 'm>,
    ) -> GameManager<'m> {
        GameManager {
            canvas,
            screen,
            event_pump,
            font,
            texture_creator,
            close_application: false,
        }
    }

    pub(crate) fn gameloop(&mut self) {
        loop {
            if self.close_application {
                break;
            }
            for event in self.event_pump.poll_iter().collect::<Vec<_>>() {
                self.handle_event(event.clone());
                self.screen.handle_event(event.clone());
            }
            self.update();
            self.paint();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub(crate) fn handle_event(&mut self, event: Event) {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.close_application(),
            _ => {}
        };
    }

    pub(crate) fn update(&mut self) {
        if let Some(screen_event) = self.screen.update() {
            self.handle_screen_events(screen_event);
        }
    }

    fn handle_screen_events(&mut self, screen: ScreenEvent) {
        match screen {
            ScreenEvent::GoToMenu => self.swap_screen(Box::new(Menu::new())),
            ScreenEvent::GoToGame => self.swap_screen(Box::new(Game::new())),
            ScreenEvent::CloseApplication => {
                self.close_application();
            }
        };
    }

    fn close_application(&mut self) {
        self.close_application = true;
    }

    pub(crate) fn swap_screen(&mut self, screen: Box<dyn Screen>) {
        self.screen.unload();
        self.screen = screen;
    }

    pub(crate) fn paint(&mut self) {
        self.screen
            .paint(self.canvas, self.font, self.texture_creator);
        self.canvas.present();
    }
}
