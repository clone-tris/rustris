use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::{Mesh, Rect};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra as na;
use ggez::{self, conf};
use ggez::{Context, GameResult};

const PLAYER_SIZE: f32 = 100.0;

enum Translate {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    x: f32,
    y: f32,
    size: f32,
}

impl Player {
    fn new(size: f32) -> Player {
        Player {
            size,
            x: 0.0,
            y: 0.0,
        }
    }
    fn center_player(&mut self, width: f32, height: f32) {
        self.x = width / 2.0 - self.size / 2.0;
        self.y = height / 2.0 - self.size / 2.0;
    }
}

struct MainState {
    player: Player,
    width: f32,
    height: f32,
}

impl MainState {
    fn new(width: f32, height: f32) -> GameResult<MainState> {
        let mut player = Player::new(100.0);

        player.center_player(width, height);

        let s = MainState {
            width,
            height,
            player,
        };
        Ok(s)
    }

    fn translate(&mut self, direction: Translate, distance: f32) {
        match direction {
            Translate::Up => {
                self.player.y += -1.0 * distance;
            }
            Translate::Down => {
                self.player.y += distance;
            }
            Translate::Left => {
                self.player.x += -1.0 * distance;
            }
            Translate::Right => {
                self.player.x += distance;
            }
        }
        if self.player.x < -self.player.size {
            self.player.x = self.width;
        } else if self.player.x > self.width {
            self.player.x = -self.player.size;
        }
        if self.player.y < -self.player.size {
            self.player.y = self.height;
        } else if self.player.y > self.height {
            self.player.y = -self.player.size;
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.translate(Translate::Up, 5.0);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.translate(Translate::Left, 5.0);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.translate(Translate::Down, 5.0);
        }
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.translate(Translate::Right, 5.0);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let bounds = Rect::new(0.0, 0.0, self.player.size, self.player.size);

        let shape = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            graphics::WHITE,
        )?;

        graphics::draw(
            ctx,
            &shape,
            (na::Point2::new(self.player.x, self.player.y),),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        _: KeyMods,
        _: bool,
    ) {
        match key {
            KeyCode::Escape => {
                ggez::event::quit(ctx);
            }
            KeyCode::Right => {
                self.translate(Translate::Right, self.player.size / 2.0)
            }
            KeyCode::Left => {
                self.translate(Translate::Left, self.player.size / 2.0)
            }
            KeyCode::Up => {
                self.translate(Translate::Up, self.player.size / 2.0)
            }
            KeyCode::Down => {
                self.translate(Translate::Down, self.player.size / 2.0)
            }
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let state = &mut MainState::new(800.0, 460.0)?;

    let cb = ggez::ContextBuilder::new("rustris", "Abderrahmane Tahri Jouti")
        .window_setup(
            conf::WindowSetup::default()
                .title("Clonetris, made easy, in Rust!"),
        )
        .window_mode(
            conf::WindowMode::default().dimensions(state.width, state.height),
        );

    let (ctx, event_loop) = &mut cb.build()?;
    event::run(ctx, event_loop, state)
}
