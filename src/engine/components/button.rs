use crate::colors::UiColors;
use crate::engine::game_painter::texture_for_text;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

const BUTTON_PADDING_LEFT: u32 = 8;
const BUTTON_PADDING_TOP: u32 = 8;

pub struct Button {
    position: Point,
}

impl Button {
    pub(crate) fn new(position: Point) -> Button {
        Button { position }
    }

    pub(crate) fn draw(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        text: String,
    ) {
        let texture =
            texture_for_text(&font, &texture_creator, &text, UiColors::ButtonText.value());
        let TextureQuery {
            width: text_width,
            height: text_height,
            ..
        } = texture.query();

        let width = 2 * BUTTON_PADDING_LEFT + text_width;
        let height = 2 * BUTTON_PADDING_TOP + text_height;

        let text_x = self.position.x + ((width - text_width) / 2) as i32;
        let text_y = self.position.y + ((height - text_height) / 2) as i32;

        canvas.set_draw_color(UiColors::ButtonBackground.value());
        canvas
            .fill_rect(Rect::new(self.position.x, self.position.y, width, height))
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(text_x, text_y, text_width, text_height),
            )
            .unwrap();
    }
}
