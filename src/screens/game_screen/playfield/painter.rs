use crate::framework::graphics_painter;
use crate::rustris_config::CANVAS_HEIGHT;
use crate::screens::game_screen::colors::{ShapeColors, UiColors};
use crate::screens::game_screen::config::{SQUARE_BORDER_WIDTH, SQUARE_WIDTH, WAR_ZONE_WIDTH};
use crate::screens::game_screen::shape::Shape;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::{graphics, Context, GameResult};
use nalgebra::Point2;

pub struct Painter {
    width: u16,
    height: u16,
}

impl Painter {
    pub fn new(width: u16, height: u16) -> Painter {
        Painter { width, height }
    }

    pub fn setup(&self, ctx: &mut Context, canvas: &Canvas) {
        graphics::set_canvas(ctx, Some(canvas));
        graphics::set_screen_coordinates(
            ctx,
            [0.0, 0.0, WAR_ZONE_WIDTH as f32, CANVAS_HEIGHT as f32].into(),
        )
        .unwrap();
    }

    pub fn clear(&self, ctx: &mut Context) {
        graphics::clear(ctx, UiColors::Background.value());
    }

    pub fn draw_shape(&self, ctx: &mut Context, shape: &Shape) {
        for square in shape.grid.iter() {
            self.draw_square_at(
                ctx,
                shape.row + square.row,
                shape.column + square.column,
                square.color,
            );
        }
    }

    pub fn draw_square_at(&self, ctx: &mut Context, row: u16, column: u16, color: Color) {
        self.draw_tetromino_square(
            ctx,
            column as u16 * SQUARE_WIDTH,
            row as u16 * SQUARE_WIDTH,
            color,
        );
    }

    pub fn draw_tetromino_square(&self, ctx: &mut Context, x: u16, y: u16, color: Color) {
        let x = x as f32;
        let y = y as f32;
        let square_width = SQUARE_WIDTH as f32;
        let square_border_width = SQUARE_BORDER_WIDTH as f32;

        let mb = &mut MeshBuilder::new();

        // Background
        mb.rectangle(
            DrawMode::fill(),
            Rect::new(x as f32, y as f32, SQUARE_WIDTH as f32, SQUARE_WIDTH as f32),
            color,
        );

        // Left Border
        mb.polygon(
            DrawMode::fill(),
            &[
                Point2::new(x, y),
                Point2::new(x + square_border_width, y + square_border_width),
                Point2::new(
                    x + square_border_width,
                    y + square_width - square_border_width,
                ),
                Point2::new(x, y + square_width),
            ],
            ShapeColors::BorderSide.value(),
        )
        .unwrap();

        // Right Border
        mb.polygon(
            DrawMode::fill(),
            &[
                Point2::new(x + square_width, y),
                Point2::new(
                    x + square_width - square_border_width,
                    y + square_border_width,
                ),
                Point2::new(
                    x + square_width - square_border_width,
                    y + square_width - square_border_width,
                ),
                Point2::new(x + square_width, y + square_width),
            ],
            ShapeColors::BorderSide.value(),
        )
        .unwrap();

        // Top Border
        mb.polygon(
            DrawMode::fill(),
            &[
                Point2::new(x, y),
                Point2::new(x + square_border_width, y + square_border_width),
                Point2::new(
                    x + square_width - square_border_width,
                    y + square_border_width,
                ),
                Point2::new(x + square_width, y),
            ],
            ShapeColors::BorderTop.value(),
        )
        .unwrap();

        // Bottom Border
        mb.polygon(
            DrawMode::fill(),
            &[
                Point2::new(x, y + square_width),
                Point2::new(
                    x + square_border_width,
                    y + square_width - square_border_width,
                ),
                Point2::new(
                    x + square_width - square_border_width,
                    y + square_width - square_border_width,
                ),
                Point2::new(x + square_width, y + square_width),
            ],
            ShapeColors::BorderBottom.value(),
        )
        .unwrap();

        let mesh = mb.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, DrawParam::default()).unwrap();
    }

    pub fn draw_guide(&self, ctx: &mut Context) {
        graphics_painter::draw_guide(ctx, 0, 0, self.width, self.height);
    }
}
