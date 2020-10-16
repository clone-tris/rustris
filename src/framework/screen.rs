use ggez::graphics::Canvas;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::Context;

pub trait Screen {
    fn update(&mut self, _ctx: &mut Context) -> Option<Box<dyn Screen>> {
        None
    }
    fn paint(&mut self, ctx: &mut Context);
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _key: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }
    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}
    fn canvas(&self) -> &Canvas;
    fn unload(&mut self, _ctx: &mut Context) {}
}
