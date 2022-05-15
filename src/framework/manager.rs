use crate::framework::change_screen::ScreenChange;
use crate::framework::screen::Screen;
use sdl2::event::Event;

pub struct Manager<'t> {
    screen: Box<dyn Screen + 't>,
}

impl<'t> Manager<'t> {
    pub(crate) fn new(initial_screen: Box<dyn Screen + 't>) -> Manager<'t> {
        Manager {
            screen: initial_screen,
        }
    }

    pub(crate) fn handle_event(&mut self, event: Event) {
        self.screen.handle_event(event);
    }

    pub(crate) fn update(&mut self) {
        if let Some(next_screen) = self.screen.update() {}
    }

    // fn spaw_screen(&mut self, screen: ScreenChange) -> Box<dyn Screen> {
    //     match screen {
    //         ScreenChange::Game => println!("Going to gae screen"),
    //         ScreenChange::Menu => {}
    //         ScreenChange::Over => {}
    //     }
    // }

    pub(crate) fn paint(&mut self) {
        self.screen.paint();
        self.screen.get_canvas().present();
    }
}
