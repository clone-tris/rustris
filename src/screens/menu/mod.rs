use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

struct Player {
    x: f32,
    y: f32,
}
pub struct Menu {
    player: Player,
    goto_game: bool,
    close_application: bool,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            player: Player {
                x: 700f32,
                y: 500f32,
            },
            goto_game: false,
            close_application: false,
        }
    }
}

impl Screen for Menu {
    fn paint(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(46, 2, 73));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(169, 16, 121));
        canvas
            .fill_rect(Rect::new(
                self.player.x as i32,
                self.player.y as i32,
                100,
                100,
            ))
            .unwrap();
    }

    fn update(&mut self) -> Option<ScreenEvent> {
        self.player.x -= 0.1;
        self.player.y -= 0.1;
        if self.goto_game {
            return Some(ScreenEvent::GoToGame);
        }
        if self.close_application {
            return Some(ScreenEvent::CloseApplication);
        }
        None
    }

    fn handle_event(&mut self, event: Event) {
        if matches!(
            event,
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            }
        ) {
            self.goto_game = true;
        }
        if matches!(
            event,
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            }
        ) {
            self.close_application = true;
        }
        // match event {
        //     Event::KeyDown {
        //         keycode: Some(Keycode::S),
        //         ..
        //     } => self.goto_game = true,
        //     _ => {}
        // }
    }
}
