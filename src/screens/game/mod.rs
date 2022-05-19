pub mod components;
// pub mod playfield;

use crate::colors::UiColors;
use crate::engine::game_painter;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use crate::main_config::{CANVAS_HEIGHT, WAR_ZONE_WIDTH};

struct Player {
    x: f32,
    y: f32,
}
pub struct Game {
    player: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player { x: 0f32, y: 0f32 },
        }
    }
}

impl<'t> Screen for Game {
    fn paint(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(UiColors::Background.value());
        canvas.clear();

        game_painter::draw_guide(canvas, 0, 0, WAR_ZONE_WIDTH as i32, CANVAS_HEIGHT as i32);
    }

    fn update(&mut self) -> Option<ScreenEvent> {
        self.player.x += 0.1;
        self.player.y += 0.1;
        None
    }
}
