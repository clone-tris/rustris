mod painter;

use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::{ShapeColors, UiColors};
use crate::screens::game_screen::config::PUZZLE_HEIGHT;
use crate::screens::game_screen::playfield::painter::Painter;
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::square::Square;
use crate::screens::game_screen::tetromino::random_tetromino;
use ggez::conf::NumSamples;
use ggez::graphics::Canvas;
use ggez::{graphics, Context};

pub struct PlayFieldScreen {
    painter: Painter,
    canvas: Canvas,
    goto_over_screen: bool,
    player: Shape,
    next_player: Shape,
    opponent: Shape,
}

impl PlayFieldScreen {
    pub fn new(ctx: &mut Context, width: u16, height: u16) -> PlayFieldScreen {
        PlayFieldScreen {
            canvas: graphics::Canvas::new(ctx, width, height, NumSamples::One).unwrap(),
            goto_over_screen: false,
            painter: Painter::new(width, height),
            player: random_tetromino(),
            next_player: random_tetromino(),
            opponent: Shape::new(Vec::new(), 0, 0, ShapeColors::DefaultSquareColor.value()),
        }
    }
    pub fn spawnPlayer(&mut self) {}
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
