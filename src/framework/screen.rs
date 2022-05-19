use crate::framework::screen_event::ScreenEvent;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;

pub trait Screen {
    fn paint(&mut self, canvas: &mut WindowCanvas);
    fn update(&mut self) -> Option<ScreenEvent> {
        None
    }
    fn handle_event(&mut self, _event: Event) {}
    fn unload(&mut self) {}
}
