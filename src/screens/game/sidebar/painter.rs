use crate::colors::UiColors;
use crate::engine::game_painter;
use crate::main_config::SQUARE_WIDTH;
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
        canvas.set_viewport(Rect::new(0, 0, self.width as u32, self.height as u32));
    }

    pub fn background(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(UiColors::SidebarBackground.value());
        canvas
            .fill_rect(Rect::new(0, 0, self.width as u32, self.height as u32))
            .unwrap();
    }

    pub fn draw_guide(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(UiColors::Guide.value());
        game_painter::draw_guide(
            canvas,
            SQUARE_WIDTH,
            SQUARE_WIDTH,
            SQUARE_WIDTH * 4,
            SQUARE_WIDTH * 2,
        );
    }
}
