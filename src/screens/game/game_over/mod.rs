use crate::engine::components::button::Button;
use crate::engine::screen_event::ScreenEvent;
use crate::engine::screen_event::ScreenEvent::{GoToGame, GoToMenu};
use crate::game_config::SQUARE_WIDTH;
use crate::{CANVAS_HEIGHT, CANVAS_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub struct Over {
    pub restart_button: Button,
    pub quit_button: Button,
}

impl Over {
    pub fn new() -> Over {
        Over {
            restart_button: Button::new(Point::new(4 * SQUARE_WIDTH, 17 * SQUARE_WIDTH)),
            quit_button: Button::new(Point::new(8 * SQUARE_WIDTH, 17 * SQUARE_WIDTH)),
        }
    }

    pub fn paint(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        canvas.set_viewport(Rect::new(0, 0, CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32));
        self.restart_button
            .draw(canvas, font, texture_creator, String::from("[R]etart"));
        self.quit_button
            .draw(canvas, font, texture_creator, String::from("[M]enu"));
    }

    pub fn handle_event(&mut self, event: Event) -> Option<ScreenEvent> {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => Some(GoToGame),
            Event::KeyDown {
                keycode: Some(Keycode::M),
                ..
            } => Some(GoToMenu),
            Event::MouseButtonDown { x, y, .. } => {
                if self.restart_button.contains(Point::new(x, y)) {
                    return Some(GoToGame);
                }
                if self.quit_button.contains(Point::new(x, y)) {
                    return Some(GoToMenu);
                }
                None
            }
            _ => None,
        }
    }
}
