use crate::colors::UiColors;
use crate::engine::components::button::Button;
use crate::engine::game_painter;
use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use crate::main_config::SQUARE_WIDTH;
use crate::{CANVAS_HEIGHT, CANVAS_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub struct Over {
    restart_button: Button,
    quit_button: Button,
    goto_game: bool,
    quit_game: bool,
    rendered: bool,
}

impl Over {
    pub fn new() -> Over {
        Over {
            restart_button: Button::new(Point::new(4 * SQUARE_WIDTH, 17 * SQUARE_WIDTH)),
            quit_button: Button::new(Point::new(8 * SQUARE_WIDTH, 17 * SQUARE_WIDTH)),
            goto_game: false,
            quit_game: false,
            rendered: false,
        }
    }
}

impl Screen for Over {
    fn paint(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        canvas.set_viewport(Rect::new(0, 0, CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32));
        canvas.set_draw_color(UiColors::Background.value());
        canvas.clear();
        self.restart_button
            .draw(canvas, font, texture_creator, String::from("[R]etart"));
        self.quit_button
            .draw(canvas, font, texture_creator, String::from("[Q]uit"));
        self.rendered = true
    }

    fn update(&mut self) -> Option<ScreenEvent> {
        if self.goto_game {
            return Some(ScreenEvent::GoToGame);
        }
        if self.quit_game {
            return Some(ScreenEvent::CloseApplication);
        }
        None
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => self.goto_game = true,
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => self.quit_game = true,
            Event::MouseButtonDown { x, y, .. } => {
                if self.restart_button.contains(Point::new(x, y)) {
                    self.goto_game = true;
                }
                if self.quit_button.contains(Point::new(x, y)) {
                    self.quit_game = true;
                }
            }
            _ => {}
        };
    }
}
