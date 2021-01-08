mod painter;

use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::ShapeColors;
use crate::screens::game_screen::playfield::painter::Painter;
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::square::Square;
use ggez::conf::NumSamples;
use ggez::graphics::Canvas;
use ggez::{graphics, Context};

pub struct PlayFieldScreen {
    painter: Painter,
    canvas: Canvas,
    goto_over_screen: bool,
    player: Shape,
}

impl PlayFieldScreen {
    pub fn new(ctx: &mut Context, width: u16, height: u16) -> PlayFieldScreen {
        println!("PlayFieldScreen, w: {} h: {}", width, height);
        PlayFieldScreen {
            canvas: graphics::Canvas::new(ctx, width, height, NumSamples::One).unwrap(),
            goto_over_screen: false,
            painter: Painter::new(width, height),
            player: Shape::new(
                vec![
                    Square::default(0, 0),
                    Square::default(1, 0),
                    Square::default(1, 1),
                    Square::default(1, 2),
                ],
                2,
                2,
                ShapeColors::Cyan.value(),
            ),
        }
    }
}

impl Screen for PlayFieldScreen {
    fn paint(&mut self, ctx: &mut Context) {
        self.painter.setup(ctx, &self.canvas);
        self.painter.clear(ctx);
        self.painter.draw_guide(ctx);
        self.painter.draw_shape(ctx, &self.player);
    }

    fn canvas(&self, _ctx: &mut Context) -> &Canvas {
        &self.canvas
    }
}
