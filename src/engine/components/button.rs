use crate::colors::UiColors;
use crate::engine::game_painter::texture_for_text;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

const BUTTON_PADDING_LEFT: u32 = 8;
const BUTTON_PADDING_TOP: u32 = 8;

pub struct Button {
    area: Rect,
}

impl Button {
    pub(crate) fn new(position: Point) -> Button {
        Button {
            area: Rect::new(position.x, position.y, 0, 0),
        }
    }

    pub(crate) fn draw(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        text: String,
    ) -> Rect {
        let texture =
            texture_for_text(&font, &texture_creator, &text, UiColors::ButtonText.value());
        let TextureQuery {
            width: text_width,
            height: text_height,
            ..
        } = texture.query();

        self.area.set_width(2 * BUTTON_PADDING_LEFT + text_width);
        self.area.set_height(2 * BUTTON_PADDING_TOP + text_height);

        let text_x = self.area.x + ((self.area.width() - text_width) / 2) as i32;
        let text_y = self.area.y + ((self.area.height() - text_height) / 2) as i32;

        canvas.set_draw_color(UiColors::ButtonBackground.value());
        let button_area = Rect::new(
            self.area.x,
            self.area.y,
            self.area.width(),
            self.area.height(),
        );
        canvas.fill_rect(button_area).unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(text_x, text_y, text_width, text_height),
            )
            .unwrap();
        button_area
    }

    pub fn contains(self: &Self, point: Point) -> bool {
        return point.x >= self.area.x
            && point.x <= self.area.x + self.area.width() as i32
            && point.y >= self.area.y
            && point.y <= self.area.y + self.area.height() as i32;
    }
}
