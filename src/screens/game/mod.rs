use crate::framework::change_screen::ScreenChange;
use crate::framework::screen::Screen;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

struct Player {
    x: f32,
    y: f32,
}
pub struct Game<'t> {
    canvas: &'t mut WindowCanvas,
    player: Player,
}

impl<'t> Game<'t> {
    pub fn new(canvas: &'t mut WindowCanvas) -> Game<'t> {
        Game {
            canvas,
            player: Player { x: 0f32, y: 0f32 },
        }
    }
}

impl<'t> Screen for Game<'t> {
    fn paint(&mut self) {
        self.canvas.set_draw_color(Color::RGB(67, 139, 220));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(67, 220, 139));
        self.canvas
            .fill_rect(Rect::new(
                self.player.x as i32,
                self.player.y as i32,
                140,
                140,
            ))
            .unwrap();
    }

    fn get_canvas(&mut self) -> &mut WindowCanvas {
        self.canvas
    }

    fn update(&mut self) -> Option<ScreenChange> {
        self.player.x += 0.1;
        self.player.y += 0.1;
        None
    }
}
