use crate::colors::UiColors;
use crate::engine::components::button::Button;
use crate::engine::game_painter::texture_for_text;
use crate::engine::screen_event::ScreenEvent;
use crate::engine::screen_event::ScreenEvent::{GoToGame, GoToMenu};
use crate::game_config::SQUARE_WIDTH;
use crate::{CANVAS_HEIGHT, CANVAS_WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
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
        self.draw_popup(canvas, font, texture_creator);
        self.restart_button
            .draw(canvas, font, texture_creator, String::from("[R]etart"));
        self.quit_button
            .draw(canvas, font, texture_creator, String::from("[M]enu"));
    }

    fn draw_popup(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let padding = 20;
        let message = "Game Over!";
        let texture = texture_for_text(
            font,
            texture_creator,
            &String::from(message),
            UiColors::PopupText.value(),
        );
        let TextureQuery {
            width: text_width,
            height: text_height,
            ..
        } = texture.query();

        let box_width = padding * 2 + text_width;
        let box_height = padding * 2 + 18;
        let box_x = (CANVAS_WIDTH - box_width as i32) / 2;
        let box_y = (CANVAS_HEIGHT - box_height as i32) / 3;

        canvas.set_draw_color(UiColors::PopupBackground.value());
        canvas
            .fill_rect(Rect::new(box_x, box_y, box_width, box_height))
            .unwrap();

        canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    box_x + padding as i32,
                    box_x + padding as i32,
                    text_width,
                    text_height,
                ),
            )
            .unwrap();
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
