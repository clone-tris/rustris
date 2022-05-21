use crate::engine::screen_event::ScreenEvent;
use sdl2::event::Event;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub trait Screen {
    fn paint(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    );
    fn update(&mut self) -> Option<ScreenEvent> {
        None
    }
    fn handle_event(&mut self, _event: Event) {}
    fn unload(&mut self) {}
}
