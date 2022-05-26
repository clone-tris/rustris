mod painter;

use crate::colors::UiColors;
use crate::engine::game_painter;
use crate::screens::game::components::score::Score;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::screens::game::components::shape::Shape;

pub struct Sidebar {
    pub next_player: Shape,
    width: i32,
    height: i32,
}

impl Sidebar {
    pub fn new(width: i32, height: i32, next_player: Shape) -> Sidebar {
        Sidebar {
            next_player,
            width,
            height,
        }
    }
}

impl Sidebar {
    pub fn paint(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        score: Score,
    ) {
        game_painter::set_viewport(canvas, 0, 0, self.width, self.height);
        game_painter::draw_background(
            canvas,
            self.width,
            self.height,
            UiColors::SidebarBackground.value(),
        );
        self.next_player.draw_at(canvas, 1, 1);
        painter::draw_score(canvas, font, texture_creator, score);
    }
}
