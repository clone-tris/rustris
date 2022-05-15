use crate::framework::screen::Screen;
use crate::framework::screen_name::ScreenName;
use crate::{Game, Menu};
use sdl2::render::WindowCanvas;

pub struct Manager<'m> {
    canvas: &'m mut WindowCanvas,
    screen: Option<Box<dyn Screen>>,
}

impl<'m> Manager<'m> {
    pub(crate) fn new(canvas: &'m mut WindowCanvas) -> Manager<'m> {
        Manager {
            canvas,
            screen: None,
        }
    }

    pub(crate) fn start(&'m mut self, initial_screen: ScreenName) {
        self.screen = Some(self.spaw_screen(initial_screen));
    }

    // pub(crate) fn handle_event(&mut self, event: Event) {
    //     self.screen.handle_event(event);
    // }

    // pub(crate) fn update(&mut self) {
    //     if let Some(next_screen) = self.screen.update() {}
    // }

    fn spaw_screen(&'m mut self, screen: ScreenName) -> Box<dyn Screen + 'm> {
        match screen {
            ScreenName::Menu => Box::new(Menu::new(self.canvas)),
            ScreenName::Game => Box::new(Game::new(self.canvas)),
        }
    }

    pub(crate) fn paint(&mut self) {
        // self.screen.paint();
        // self.screen.get_canvas().present();
    }
}
