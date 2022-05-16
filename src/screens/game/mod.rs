use crate::framework::screen::Screen;
use crate::framework::screen_name::ScreenName;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

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
        canvas.set_draw_color(Color::RGB(67, 139, 220));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(67, 220, 139));
        canvas
            .fill_rect(Rect::new(
                self.player.x as i32,
                self.player.y as i32,
                140,
                140,
            ))
            .unwrap();
    }

    fn update(&mut self) -> Option<ScreenName> {
        self.player.x += 0.1;
        self.player.y += 0.1;
        None
    }
}
