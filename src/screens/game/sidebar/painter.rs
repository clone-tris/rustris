use crate::colors::UiColors;
use crate::engine::game_painter::draw_text;
use crate::game_config::SQUARE_WIDTH;
use crate::screens::game::components::score::Score;
use sdl2::rect::Point;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub fn draw_score(
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
        &format!("Level {:>9}", score.level),
        UiColors::WhiteText.value(),
    );
    draw_text(
        canvas,
        font,
        texture_creator,
        Point::new(SQUARE_WIDTH / 3, SQUARE_WIDTH * 5),
        &format!("Cleared {:>7}", score.lines_cleared),
        UiColors::WhiteText.value(),
    );
    draw_text(
        canvas,
        font,
        texture_creator,
        Point::new(SQUARE_WIDTH / 3, SQUARE_WIDTH * 6),
        &format!("Score {:>9}", score.total),
        UiColors::WhiteText.value(),
    );
}
