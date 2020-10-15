use ggez::graphics::Canvas;
use ggez::Context;

pub trait Screen {
    fn update(&self, ctx: &mut Context);
    fn paint(&self, ctx: &mut Context);
    fn get_canvas(&self) -> &Canvas;
}
