mod graphic;

use crate::colors::UiColors;
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
    goto_game: bool,
    graphic: Shape,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            goto_game: false,
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
        game_painter::draw_button(
            canvas,
            font,
            texture_creator,
            Point::new(6 * SQUARE_WIDTH, 17 * SQUARE_WIDTH),
            String::from("Start (S)"),
        );
    }

    fn update(&mut self) -> Option<ScreenEvent> {
        if self.goto_game {
            return Some(ScreenEvent::GoToGame);
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
        };
    }
}
