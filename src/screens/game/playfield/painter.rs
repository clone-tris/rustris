use crate::colors::UiColors;
use crate::engine::game_painter;
use crate::main_config::{SIDEBAR_WIDTH, SQUARE_BORDER_WIDTH, SQUARE_WIDTH};
use crate::screens::game::components::shape::Shape;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Painter {
    width: i32,
    height: i32,
}

impl Painter {
    pub fn new(width: i32, height: i32) -> Painter {
        Painter { width, height }
    }

    pub fn setup(&self, canvas: &mut WindowCanvas) {
        canvas.set_viewport(Rect::new(
            SIDEBAR_WIDTH,
            0,
            self.width as u32,
            self.height as u32,
        ));
    }

    pub fn background(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(UiColors::Background.value());
        canvas
            .fill_rect(Rect::new(0, 0, self.width as u32, self.height as u32))
            .unwrap();
    }

    pub fn draw_guide(&self, canvas: &mut WindowCanvas) {
        game_painter::draw_guide(canvas, 0, 0, self.width, self.height);
    }
}
