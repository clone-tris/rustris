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

pub struct SdlContext<'a> {
    canvas: &'a mut WindowCanvas,
    font: &'a Font<'a, 'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
    event_pump: &'a mut EventPump,
}

pub struct GameManager<'a> {
    screen: Box<dyn Screen + 'a>,
    close_application: bool,
    sdl_context: SdlContext<'a>,
}

impl<'a> GameManager<'a> {
    pub(crate) fn new(
        screen: Box<dyn Screen + 'a>,
        canvas: &'a mut WindowCanvas,
        event_pump: &'a mut EventPump,
        texture_creator: &'a TextureCreator<WindowContext>,
        font: &'a Font<'a, 'a>,
    ) -> GameManager<'a> {
        GameManager {
            close_application: false,
            screen,
            sdl_context: SdlContext {
                canvas,
                event_pump,
                texture_creator,
                font,
            },
        }
    }

    pub(crate) fn gameloop(&mut self) {
        loop {
            if self.close_application {
                break;
            }
            for event in self.sdl_context.event_pump.poll_iter().collect::<Vec<_>>() {
                self.handle_event(event.clone());
                self.screen.handle_event(event.clone());
            }
            self.update();
            self.paint();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub(crate) fn update(&mut self) {
        if let Some(screen_event) = self.screen.update() {
            self.handle_screen_events(screen_event);
        }
    }

    pub(crate) fn paint(&mut self) {
        self.screen.paint(
            self.sdl_context.canvas,
            self.sdl_context.font,
            self.sdl_context.texture_creator,
        );
        self.sdl_context.canvas.present();
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
}
