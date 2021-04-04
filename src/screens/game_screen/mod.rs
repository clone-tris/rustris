pub mod colors;
pub mod config;
pub mod painter;
pub mod playfield;
pub mod shape;
pub mod sidebar;
pub mod square;
pub mod tetromino;

use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context};

use crate::framework::screen::Screen;
use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::config::WAR_ZONE_WIDTH;
use crate::screens::game_screen::playfield::PlayFieldScreen;
use crate::screens::game_screen::tetromino::{random_tetromino, Tetromino};
use crate::screens::over_screen::OverScreen;
use ggez::conf::NumSamples;
use std::time::{Duration, Instant};

pub struct GameScreen {
    canvas: Canvas,
    playfield: PlayFieldScreen,
    goto_over_screen: bool,
    player_is_falling: bool,
    paused: bool,
    next_fall: Instant,
    remaining_after_paused: Duration,
}

impl GameScreen {
    pub fn new(ctx: &mut Context) -> GameScreen {
        GameScreen {
            canvas: graphics::Canvas::new(
                ctx,
                CANVAS_WIDTH as u16,
                CANVAS_HEIGHT as u16,
                NumSamples::One,
            )
            .unwrap(),
            playfield: PlayFieldScreen::new(ctx, WAR_ZONE_WIDTH, CANVAS_HEIGHT),
            goto_over_screen: false,
            player_is_falling: false,
            paused: false,
            next_fall: Instant::now(),
            remaining_after_paused: Duration::from_millis(0),
        }
    }

    pub fn apply_gravity(&mut self) {
        let now = Instant::now();

        if (now >= self.next_fall) {
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

impl Screen for GameScreen {
    fn update(&mut self, ctx: &mut Context) -> Option<Box<dyn Screen>> {
        if self.paused {
            return None;
        }
        if self.goto_over_screen {
            return Some(Box::new(OverScreen::new(ctx)));
        }
        self.apply_gravity();
        None
    }

    fn paint(&mut self, ctx: &mut Context) {
        self.playfield.paint(ctx);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode) {
        match key {
            // KeyCode::O => self.goto_over_screen = true,
            KeyCode::W => self.playfield.rotate_player(),
            KeyCode::A => self.playfield.move_left(),
            KeyCode::S => self.make_player_fall(),
            KeyCode::D => self.playfield.move_right(),
            KeyCode::P => self.paused = !self.paused,
            _ => (),
        }
    }

    fn canvas(&self, ctx: &mut Context) -> &Canvas {
        let playfield_canvas = &self.playfield.canvas(ctx);
        painter::stitch(ctx, &self.canvas, playfield_canvas);
        &self.canvas
    }
}
