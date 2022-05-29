pub mod components;
pub mod playfield;
pub mod sidebar;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::time::{Duration, Instant};

use crate::engine::screen::Screen;
use crate::engine::screen_event::ScreenEvent;
use crate::game_config::{CANVAS_HEIGHT, SIDEBAR_WIDTH, WAR_ZONE_WIDTH};
use crate::screens::game::playfield::PlayField;
use crate::screens::game::sidebar::Sidebar;

pub struct Game {
    playfield: PlayField,
    sidebar: Sidebar,
    player_is_falling: bool,
    paused: bool,
    next_fall: Instant,
    remaining_after_paused: Duration,
    goto_over_screen: bool,
    restart: bool,
}

impl Game {
    pub fn new() -> Game {
        let playfield = PlayField::new(WAR_ZONE_WIDTH, CANVAS_HEIGHT);
        let next_player = playfield.next_player.clone();
        Game {
            playfield,
            sidebar: Sidebar::new(SIDEBAR_WIDTH, CANVAS_HEIGHT, next_player),
            player_is_falling: false,
            paused: false,
            next_fall: Instant::now(),
            remaining_after_paused: Duration::from_millis(0),
            goto_over_screen: false,
            restart: false,
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
            self.next_fall = self.playfield.end_of_lock;
            self.sidebar.next_player = self.playfield.next_player.clone();
        }
        self.player_is_falling = false
    }

    pub fn toggle_paused(&mut self) {
        self.paused = !self.paused;
        if self.paused {
            let now = Instant::now();
            self.remaining_after_paused = if now < self.next_fall {
                self.next_fall - now
            } else {
                Duration::from_secs(0)
            };
        }
    }

    pub fn restart(&mut self) {
        self.restart = true;
        self.playfield.reset_score();
    }
}

impl<'t> Screen for Game {
    fn paint(
        &mut self,
        canvas: &mut WindowCanvas,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        if self.goto_over_screen {
            return;
        }

        self.playfield.paint(canvas);
        self.sidebar
            .paint(canvas, font, texture_creator, self.playfield.score);
    }

    fn update(&mut self) -> Option<ScreenEvent> {
        if self.paused {
            return None;
        }
        if self.goto_over_screen {
            return Some(ScreenEvent::GoToOver);
        }
        if self.restart {
            return Some(ScreenEvent::GoToGame);
        }
        self.apply_gravity();
        None
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => self.playfield.rotate_player(),
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
                keycode: Some(Keycode::R),
                ..
            } => self.restart(),
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => self.toggle_paused(),

            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => self.goto_over_screen = true,

            _ => {}
        };
    }
}
