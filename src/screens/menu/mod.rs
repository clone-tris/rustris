use crate::framework::screen::Screen;
use crate::framework::screen_name::ScreenName;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

struct Player {
    x: f32,
    y: f32,
}
pub struct Menu<'t> {
    canvas: &'t mut WindowCanvas,
    player: Player,
    goto_game: bool,
}

impl<'t> Menu<'t> {
    pub fn new(canvas: &'t mut WindowCanvas) -> Menu {
        Menu {
            canvas,
            player: Player {
                x: 700f32,
                y: 500f32,
            },
            goto_game: false,
        }
    }
}

impl<'t> Screen for Menu<'t> {
    fn paint(&mut self) {
        self.canvas.set_draw_color(Color::RGB(46, 2, 73));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(169, 16, 121));
        self.canvas
            .fill_rect(Rect::new(
                self.player.x as i32,
                self.player.y as i32,
                100,
                100,
            ))
            .unwrap();
    }

    fn get_canvas(&mut self) -> &mut WindowCanvas {
        self.canvas
    }

    fn update(&mut self) -> Option<ScreenName> {
        self.player.x -= 0.1;
        self.player.y -= 0.1;
        if self.goto_game {
            // return Some(ScreenName::Game);
        } else {
            // return None;
        }
        None
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => self.goto_game = true,
            _ => {}
        }
    }
}
