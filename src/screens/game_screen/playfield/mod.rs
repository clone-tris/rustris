mod painter;

use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::{shaope_color, ShapeColors};
use crate::screens::game_screen::playfield::painter::Painter;
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::square::Square;
use crate::screens::game_screen::GameScreen;
use crate::screens::over_screen::OverScreen;
use ggez::conf::NumSamples;
use ggez::graphics::Canvas;
use ggez::input::keyboard::KeyCode;
use ggez::{graphics, Context};

pub struct PlayFieldScreen {
    painter: Painter,
    canvas: Canvas,
    goto_over_screen: bool,
}

impl PlayFieldScreen {
    pub fn new(ctx: &mut Context, width: u16, height: u16) -> PlayFieldScreen {
        println!("PlayFieldScreen, w: {} h: {}", width, height);
        PlayFieldScreen {
            canvas: graphics::Canvas::new(ctx, width, height, NumSamples::One).unwrap(),
            goto_over_screen: false,
            painter: Painter::new(width, height),
        }
    }
}

impl Screen for PlayFieldScreen {
    fn paint(&mut self, ctx: &mut Context) {
        graphics::set_canvas(ctx, Some(&self.canvas));
        self.painter.clear(ctx);
        self.painter.draw_guide(ctx);

        let shape = Shape::new(
            vec![
                Square::default(0, 0),
                Square::default(1, 0),
                Square::default(1, 1),
                Square::default(1, 2),
            ],
            2,
            2,
            shaope_color(ShapeColors::Cyan),
        );

        self.painter.draw_shape(ctx, shape)
    }

    fn canvas(&self, _ctx: &mut Context) -> &Canvas {
        &self.canvas
    }
}
