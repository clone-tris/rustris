use crate::colors::UiColors;
use crate::engine::game_painter::draw_text;
use crate::main_config::SQUARE_WIDTH;
use crate::screens::game::components::score::Score;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

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

    pub fn draw_score(
        &self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        score: Score,
    ) {
        draw_text(
            canvas,
            font,
            texture_creator,
            Point::new(SQUARE_WIDTH / 3, SQUARE_WIDTH * 4),
            format!("Level {:>9}", score.level),
            UiColors::WhiteText.value(),
        );
        draw_text(
            canvas,
            font,
            texture_creator,
            Point::new(SQUARE_WIDTH / 3, SQUARE_WIDTH * 5),
            format!("Cleared {:>7}", score.lines_cleared),
            UiColors::WhiteText.value(),
        );
        draw_text(
            canvas,
            font,
            texture_creator,
            Point::new(SQUARE_WIDTH / 3, SQUARE_WIDTH * 6),
            format!("Score {:>9}", score.total),
            UiColors::WhiteText.value(),
        );
    }
}
