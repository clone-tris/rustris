mod painter;

use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::{ShapeColors, UiColors};
use crate::screens::game_screen::config::{PUZZLE_HEIGHT, PUZZLE_WIDTH};
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
    on_floor: bool,
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

impl PlayFieldScreen {
    pub fn new(ctx: &mut Context, width: i16, height: i16) -> PlayFieldScreen {
        let mut opponent = Shape::new(Vec::new(), 0, 0, ShapeColors::DefaultSquareColor.value());
        opponent.width = PUZZLE_WIDTH;
        opponent.height = PUZZLE_HEIGHT;

        let mut screen = PlayFieldScreen {
            canvas: graphics::Canvas::new(ctx, width as u16, height as u16, NumSamples::One)
                .unwrap(),
            goto_over_screen: false,
            painter: Painter::new(width, height),
            player: random_tetromino(),
            next_player: random_tetromino(),
            opponent,
            on_floor: false
        };

        screen.spawn_player();

        screen
    }

    pub fn spawn_player(&mut self) {
        let mut player = self.next_player.clone();
        player.row -= player.height as i8;
        player.column = (PUZZLE_WIDTH - player.width) as i8 / 2;

        self.player = player;
        self.next_player = random_tetromino();
    }

    pub fn move_player(&mut self, row_direction: u8, column_direction: u8) -> bool {
        let mut foreshadow = self.player.clone();
        let moving_down = row_direction == 1;
        foreshadow.translate(row_direction, column_direction);
        let able_to_move = foreshadow.collides_with(&self.opponent) && foreshadow.within_bounds();
        if able_to_move {
            self.player = foreshadow
            if moving_down {
                self.on_floor = false
            }
        } else if moving_down {
            // handle falling down
        }

        able_to_move
    }
}
