mod painter;

use crate::engine::screen::Screen;

use crate::screens::game::sidebar::painter::Painter;
use sdl2::render::WindowCanvas;

pub struct Sidebar {
    painter: Painter,
}

impl Screen for Sidebar {
    fn paint(&mut self, canvas: &mut WindowCanvas) {
        self.painter.setup(canvas);
        self.painter.background(canvas);
        self.painter.draw_guide(canvas);
    }
}

impl Sidebar {
    pub fn new(width: i32, height: i32) -> Sidebar {
        Sidebar {
            painter: Painter::new(width, height),
        }
    }
}
