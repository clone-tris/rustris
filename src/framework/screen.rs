use crate::framework::screen_name::ScreenName;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;

pub trait Screen {
    fn paint(&mut self);
    fn get_canvas(&mut self) -> &mut WindowCanvas;
    fn update(&mut self) -> Option<ScreenName> {
        None
    }
    fn handle_event(&mut self, _event: Event) {}
    fn unload(&mut self) {}
}
