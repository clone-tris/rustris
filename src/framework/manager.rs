use crate::framework::screen::Screen;
use crate::framework::screen_name::ScreenName;
use crate::{Game, Menu};
use sdl2::event::Event;
use sdl2::render::WindowCanvas;

pub struct Manager<'m> {
    canvas: &'m mut WindowCanvas,
    screen: Box<dyn Screen + 'm>,
}

impl<'m> Manager<'m> {
    pub(crate) fn new(canvas: &'m mut WindowCanvas, screen: Box<dyn Screen + 'm>) -> Manager<'m> {
        Manager { canvas, screen }
    }

    pub(crate) fn handle_event(&mut self, event: Event) {
        self.screen.handle_event(event);
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
