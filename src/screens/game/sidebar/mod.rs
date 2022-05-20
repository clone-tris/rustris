mod painter;

use sdl2::render::WindowCanvas;

use crate::engine::screen::Screen;
use crate::screens::game::components::shape::Shape;
use crate::screens::game::components::tetromino::random_tetromino;
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

impl Screen for Sidebar {
    fn paint(&mut self, canvas: &mut WindowCanvas) {
        self.painter.setup(canvas);
        self.painter.background(canvas);
        // self.painter.draw_guide(canvas);
        self.next_player.draw_at(canvas, 1, 1);
    }
}
