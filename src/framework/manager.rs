use crate::framework::screen::Screen;

pub struct Manager<'t> {
    screen: Box<dyn Screen + 't>,
}
impl<'t> Manager<'t> {
    pub(crate) fn new(initial_screen: Box<dyn Screen + 't>) -> Manager<'t> {
        Manager {
            screen: initial_screen,
        }
    }

    pub(crate) fn update(&mut self) {
        self.screen.update();
    }

    pub(crate) fn paint(&mut self) {
        self.screen.paint();
        self.screen.get_canvas().present();
    }
}
