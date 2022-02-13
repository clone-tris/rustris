use crate::rustris_config::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::screens::game_screen::colors::{ShapeColors, UiColors};
use crate::screens::game_screen::config::{SIDEBAR_WIDTH, WAR_ZONE_WIDTH};
use ggez::conf::NumSamples;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, DrawParam, get_window_color_format, Mesh};
use ggez::{graphics, Context, GameResult, event};
use glam;

pub struct Test {
    left_canvas: Canvas,
    right_canvas: Canvas,
}

impl Test {
    pub fn new(ctx: &mut Context) -> GameResult<Test> {
        let state = Test {
            left_canvas: graphics::Canvas::new(
                ctx,
                SIDEBAR_WIDTH as u16,
                CANVAS_HEIGHT as u16,
                NumSamples::One,
                get_window_color_format(ctx),
            )
                .unwrap(),
            right_canvas: graphics::Canvas::new(
                ctx,
                WAR_ZONE_WIDTH as u16,
                CANVAS_HEIGHT as u16,
                NumSamples::One,
                get_window_color_format(ctx),
            )
                .unwrap(),
        };

        Ok(state)
    }

    fn draw_left(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, Some(&self.left_canvas));
        graphics::set_screen_coordinates(
            ctx,
            [0.0, 0.0, SIDEBAR_WIDTH as f32, CANVAS_HEIGHT as f32].into(),
        )
            .unwrap();

        graphics::clear(ctx, ShapeColors::Blue.value());

        let line = Mesh::new_line(
            ctx,
            &[
                glam::Vec2::new(0.0, 20.0),
                glam::Vec2::new(SIDEBAR_WIDTH as f32, 20.0),
            ],
            4.0,
            ShapeColors::Green.value(),
        )
            .unwrap();

        graphics::draw(ctx, &line, DrawParam::new()).unwrap();
    }

    fn draw_right(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, Some(&self.right_canvas));
        graphics::set_screen_coordinates(
            ctx,
            [0.0, 0.0, WAR_ZONE_WIDTH as f32, CANVAS_HEIGHT as f32].into(),
        )
            .unwrap();
        graphics::clear(ctx, ShapeColors::Red.value());

        let line = Mesh::new_line(
            ctx,
            &[
                glam::Vec2::new(0.0, 20.0),
                glam::Vec2::new(WAR_ZONE_WIDTH as f32, 50.0),
            ],
            4.0,
            ShapeColors::Orange.value(),
        )
            .unwrap();

        let right_border = Mesh::new_line(
            ctx,
            &[
                glam::Vec2::new(WAR_ZONE_WIDTH as f32 - 6.0, 0.0),
                glam::Vec2::new(WAR_ZONE_WIDTH as f32 - 6.0, CANVAS_HEIGHT as f32),
            ],
            4.0,
            ShapeColors::Cyan.value(),
        )
            .unwrap();

        graphics::draw(ctx, &line, DrawParam::new()).unwrap();
        graphics::draw(ctx, &right_border, DrawParam::new()).unwrap();
    }

    fn stitch(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, None);
        graphics::set_screen_coordinates(
            ctx,
            [
                0.0,
                0.0,
                (SIDEBAR_WIDTH + WAR_ZONE_WIDTH) as f32,
                CANVAS_HEIGHT as f32,
            ]
                .into(),
        )
            .unwrap();
        graphics::draw(
            ctx,
            &self.left_canvas,
            DrawParam::new().dest(glam::Vec2::new(0.0, 0.0)),
        )
            .unwrap();
        graphics::draw(
            ctx,
            &self.right_canvas,
            DrawParam::new().dest(glam::Vec2::new(SIDEBAR_WIDTH as f32, 0.0)),
        )
            .unwrap();
    }
}

impl event::EventHandler<ggez::GameError> for Test {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_left(ctx);
        self.draw_right(ctx);
        self.stitch(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}
