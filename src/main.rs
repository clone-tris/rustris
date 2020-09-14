use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::{Mesh, Rect};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra as na;
use ggez::{self, conf};
use ggez::{Context, GameResult};

enum Translate {
    Up,
    Down,
    Left,
    Right,
}

struct MainState {
    pos_x: f32,
    pos_y: f32,
    box_size: f32,
    win_w: f32,
    win_h: f32,
}

impl MainState {
    fn new(win_w: f32, win_h: f32) -> GameResult<MainState> {
        let box_size = 100.0;
        let s = MainState {
            win_w,
            win_h,
            pos_x: win_w / 2.0 - box_size / 2.0,
            pos_y: win_h / 2.0 - box_size / 2.0,
            box_size,
        };
        Ok(s)
    }

    fn translate(&mut self, direction: Translate, distance: f32) {
        match direction {
            Translate::Up => {
                self.pos_y += -1.0 * distance;
            }
            Translate::Down => {
                self.pos_y += distance;
            }
            Translate::Left => {
                self.pos_x += -1.0 * distance;
            }
            Translate::Right => {
                self.pos_x += distance;
            }
        }
        if self.pos_x < -self.box_size {
            self.pos_x = self.win_w;
        } else if self.pos_x > self.win_w {
            self.pos_x = -self.box_size;
        }
        if self.pos_y < -self.box_size {
            println!("x: {}, y: {}", self.pos_x, self.pos_y);
            self.pos_y = self.win_h;
            println!("x: {}, y: {}", self.pos_x, self.pos_y);
        } else if self.pos_y > self.win_h {
            println!("x: {}, y: {}", self.pos_x, self.pos_y);
            println!("x: {}, y: {}", self.pos_x, self.pos_y);
            self.pos_y = -self.box_size;
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            println!("x: {}, y: {}", self.pos_x, self.pos_y);
        }

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

        let bounds = Rect::new(0.0, 0.0, self.box_size, self.box_size);

        let shape = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            graphics::WHITE,
        )?;

        graphics::draw(
            ctx,
            &shape,
            (na::Point2::new(self.pos_x, self.pos_y),),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        mods: KeyMods,
        _: bool,
    ) {
        match key {
            KeyCode::Escape => {
                ggez::event::quit(ctx);
            }
            KeyCode::Right => {
                self.translate(Translate::Right, self.box_size / 2.0)
            }
            KeyCode::Left => {
                self.translate(Translate::Left, self.box_size / 2.0)
            }
            KeyCode::Up => self.translate(Translate::Up, self.box_size / 2.0),
            KeyCode::Down => {
                self.translate(Translate::Down, self.box_size / 2.0)
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
            conf::WindowMode::default().dimensions(state.win_w, state.win_h),
        );

    let (ctx, event_loop) = &mut cb.build()?;
    event::run(ctx, event_loop, state)
}
