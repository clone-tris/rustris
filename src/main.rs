use ggez::event::KeyCode;
use ggez::graphics;
use ggez::graphics::{DrawParam, Font, Mesh, Rect};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra as na;
use ggez::{self, conf};
use ggez::{event, timer};
use ggez::{Context, GameResult};
use std::env;
use std::path;
use std::time::Duration;

const DESIRED_FPS: u32 = 60;
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 460.0;

enum Force {
    Up,
    Down,
    Left,
    Right,
    Resistence,
}

#[derive(Debug)]
struct Player {
    x: f32,
    y: f32,
    size: f32,
    acceleration: f32,
    max_velocity: f32,
    x_velocity: f32,
    y_velocity: f32,
    no_x_force: bool,
    no_y_force: bool,
}

impl Player {
    fn new(size: f32) -> Player {
        let player = Player {
            size,
            x: 0.0,
            y: 0.0,
            acceleration: 2.0,
            max_velocity: 10.0,
            x_velocity: 0.0,
            y_velocity: 0.0,
            no_x_force: true,
            no_y_force: true,
        };

        println!("{:?}", player);

        return player;
    }
    fn center_player(&mut self) {
        self.x = WIN_WIDTH / 2.0 - self.size / 2.0;
        self.y = WIN_HEIGHT / 2.0 - self.size / 2.0;
    }
}

struct MainState {
    player: Player,
    text: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut player = Player::new(100.0);

        player.center_player();

        let font = Font::new(ctx, "/SourceCodePro-Regular.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));

        let s = MainState { player, text };
        Ok(s)
    }

    fn apply_force(&mut self, direction: Force, dt: Duration) {
        let player = &mut self.player;
        let dt_f32 = dt.as_secs_f32();

        player.y += player.y_velocity;
        player.x += player.x_velocity;

        if player.x < -player.size {
            player.x = WIN_WIDTH;
        } else if player.x > WIN_WIDTH {
            player.x = -player.size;
        }
        if player.y < -player.size {
            player.y = WIN_HEIGHT;
        } else if player.y > WIN_HEIGHT {
            player.y = -player.size;
        }

        match direction {
            Force::Up => {
                player.y_velocity -= player.acceleration * dt_f32;
                player.no_y_force = false;
            }
            Force::Down => {
                player.y_velocity += player.acceleration * dt_f32;
                player.no_y_force = false;
            }
            Force::Left => {
                player.x_velocity -= player.acceleration * dt_f32;
                player.no_x_force = false;
            }
            Force::Right => {
                player.x_velocity += player.acceleration * dt_f32;
                player.no_x_force = false;
            }
            Force::Resistence => {
                if player.x_velocity != 0.0 && player.no_x_force {
                    let u_x = player.x_velocity / player.x_velocity.abs();
                    player.x_velocity -= u_x * dt_f32 * player.acceleration;
                    if u_x * player.x_velocity < 0.0 {
                        player.x_velocity = 0.0;
                    }
                }
                if player.y_velocity != 0.0 && player.no_y_force {
                    let u_y = player.y_velocity / player.y_velocity.abs();
                    player.y_velocity -= u_y * dt_f32 * player.acceleration;
                    if u_y * player.y_velocity < 0.0 {
                        player.y_velocity = 0.0;
                    }
                }
                player.no_x_force = true;
                player.no_y_force = true;
            }
        }

        if player.x_velocity > player.max_velocity {
            player.x_velocity = player.max_velocity
        }
        if player.x_velocity < -player.max_velocity {
            player.x_velocity = -player.max_velocity
        }
        if player.y_velocity > player.max_velocity {
            player.y_velocity = player.max_velocity
        }
        if player.y_velocity < -player.max_velocity {
            player.y_velocity = -player.max_velocity
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = timer::delta(ctx);

            self.apply_force(Force::Resistence, dt);

            if keyboard::is_key_pressed(ctx, KeyCode::W) {
                self.apply_force(Force::Up, dt);
            }
            if keyboard::is_key_pressed(ctx, KeyCode::A) {
                self.apply_force(Force::Left, dt);
            }
            if keyboard::is_key_pressed(ctx, KeyCode::S) {
                self.apply_force(Force::Down, dt);
            }
            if keyboard::is_key_pressed(ctx, KeyCode::D) {
                self.apply_force(Force::Right, dt);
            }
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

        let text_position = mint::Point2 { x: 5.0, y: 5.0 };
        graphics::draw(ctx, &self.text, (text_position,))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")
    {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("rustris", "Abderrahmane Tahri Jouti")
        .window_setup(
            conf::WindowSetup::default()
                .title("Clonetris, made easy, in Rust!"),
        )
        .window_mode(
            conf::WindowMode::default().dimensions(WIN_WIDTH, WIN_HEIGHT),
        )
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
