use crate::framework::screen::Screen;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Game<'t> {
    canvas: &'t mut WindowCanvas,
}

impl<'t> Game<'t> {
    pub fn new(canvas: &'t mut WindowCanvas) -> Game<'t> {
        Game { canvas }
    }
}

impl<'t> Screen for Game<'t> {
    fn paint(&mut self) {
        self.canvas.set_draw_color(Color::RGB(67, 139, 220));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(67, 220, 139));
        self.canvas.fill_rect(Rect::new(10, 10, 140, 140)).unwrap();
        self.canvas.present();
    }
}
