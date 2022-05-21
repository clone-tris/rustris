mod painter;

use crate::screens::game::components::score::Score;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::screens::game::components::shape::Shape;
use crate::screens::game::sidebar::painter::Painter;

pub struct Sidebar {
    painter: Painter,
    pub next_player: Shape,
}

impl Sidebar {
    pub fn new(width: i32, height: i32, next_player: Shape) -> Sidebar {
        Sidebar {
            painter: Painter::new(width, height),
            next_player,
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
        self.painter.setup(canvas);
        self.painter.background(canvas);
        self.next_player.draw_at(canvas, 1, 1);
        self.painter
            .draw_score(canvas, font, texture_creator, score);
    }
}
