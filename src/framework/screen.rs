use sdl2::render::WindowCanvas;

pub trait Screen {
    // todo: figure out types
    fn paint(&mut self);
    fn get_canvas(&mut self) -> &mut WindowCanvas;
    fn update(&mut self) {}
    // fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) {}
    // fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) {}
    // fn unload(&mut self, _ctx: &mut Context) {}
}
