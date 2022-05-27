mod graphic;

use crate::colors::UiColors;
use crate::engine::components::button::Button;
use crate::engine::game_painter;
use crate::{CANVAS_HEIGHT, CANVAS_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use crate::main_config::SQUARE_WIDTH;
use crate::screens::game::components::shape::Shape;
use crate::screens::menu::graphic::get_graphic;

pub struct Menu {
    start_button: Button,
    quit_button: Button,
    goto_game: bool,
    quit_game: bool,
    graphic: Shape,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            start_button: Button::new(Point::new(4 * SQUARE_WIDTH, 17 * SQUARE_WIDTH)),
            quit_button: Button::new(Point::new(8 * SQUARE_WIDTH, 17 * SQUARE_WIDTH)),
            goto_game: false,
            quit_game: false,
            graphic: Shape::new(get_graphic(), 0, 0),
        }
    }
}

impl Screen for Menu {
    fn paint(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        canvas.set_draw_color(UiColors::Background.value());
        canvas.clear();
        game_painter::draw_guide(canvas, 0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
        self.graphic.draw(canvas);
        self.start_button
            .draw(canvas, font, texture_creator, String::from("[S]tart"));
        self.quit_button
            .draw(canvas, font, texture_creator, String::from("[Q]uit"));
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
                keycode: Some(Keycode::S),
                ..
            } => self.goto_game = true,
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => self.quit_game = true,
            _ => {}
        };
    }
}
