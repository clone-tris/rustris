pub mod components;
pub mod playfield;
pub mod sidebar;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};

use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use crate::main_config::{CANVAS_HEIGHT, SIDEBAR_WIDTH, WAR_ZONE_WIDTH};
use crate::screens::game::playfield::PlayField;
use crate::screens::game::sidebar::Sidebar;

pub struct Game {
    playfield: PlayField,
    sidebar: Sidebar,
    goto_over_screen: bool,
    player_is_falling: bool,
    paused: bool,
    next_fall: Instant,
    remaining_after_paused: Duration,
}

impl Game {
    pub fn new() -> Game {
        Game {
            playfield: PlayField::new(WAR_ZONE_WIDTH, CANVAS_HEIGHT),
            sidebar: Sidebar::new(SIDEBAR_WIDTH, CANVAS_HEIGHT),
            goto_over_screen: false,
            player_is_falling: false,
            paused: false,
            next_fall: Instant::now(),
            remaining_after_paused: Duration::from_millis(0),
        }
    }

    pub fn apply_gravity(&mut self) {
        let now = Instant::now();

        if now >= self.next_fall {
            self.next_fall = now + self.playfield.fall_rate;
            self.make_player_fall();
        }
    }

    pub fn make_player_fall(&mut self) {
        if self.player_is_falling {
            return;
        }

        self.player_is_falling = true;
        let able_to_move = self.playfield.fall_down();
        if !able_to_move && self.playfield.is_game_ended() {
            self.goto_over_screen = true;
            return;
        }

        if !able_to_move && self.playfield.on_floor {
            self.next_fall = self.playfield.end_of_lock
            // update sidebar's next player
        }
        self.player_is_falling = false
    }
}

impl<'t> Screen for Game {
    fn paint(&mut self, canvas: &mut WindowCanvas) {
        self.playfield.paint(canvas);
        self.sidebar.paint(canvas);
    }

    fn update(&mut self) -> Option<ScreenEvent> {
        if self.paused {
            return None;
        }
        if self.goto_over_screen {
            return Some(ScreenEvent::GoToMenu);
        }
        self.apply_gravity();
        None
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => self.goto_over_screen = true,
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.playfield.rotate_player();
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => self.playfield.move_left(),
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => self.make_player_fall(),
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => self.playfield.move_right(),
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => self.paused = !self.paused,

            _ => {}
        };
    }
}
