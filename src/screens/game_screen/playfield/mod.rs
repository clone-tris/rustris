mod painter;

use crate::framework::screen::Screen;
use crate::screens::game_screen::colors::{ShapeColors, UiColors};
use crate::screens::game_screen::config::{PUZZLE_HEIGHT, PUZZLE_WIDTH};
use crate::screens::game_screen::playfield::painter::Painter;
use crate::screens::game_screen::shape::Shape;
use crate::screens::game_screen::tetromino::random_tetromino;
use ggez::conf::NumSamples;
use ggez::graphics::{Canvas, get_window_color_format};
use ggez::{graphics, Context};
use std::time::{Duration, Instant};

pub struct PlayFieldScreen {
    painter: Painter,
    canvas: Canvas,
    goto_over_screen: bool,
    pub on_floor: bool,
    game_ended: bool,
    floor_rate: Duration,
    pub fall_rate: Duration,
    pub end_of_lock: Instant,
    player: Shape,
    next_player: Shape,
    opponent: Shape,
}

impl Screen for PlayFieldScreen {
    fn paint(&mut self, ctx: &mut Context) {
        self.painter.setup(ctx, &self.canvas);
        self.painter.clear(ctx);
        self.painter.draw_guide(ctx);
        self.painter.draw_shape(ctx, &self.player);
        self.painter.draw_shape(ctx, &self.opponent);
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
            canvas: graphics::Canvas::new(ctx, width as u16, height as u16, NumSamples::One, get_window_color_format(ctx))
                .unwrap(),
            painter: Painter::new(width, height),
            player: random_tetromino(),
            next_player: random_tetromino(),
            opponent,
            goto_over_screen: false,
            on_floor: false,
            floor_rate: Duration::from_millis(500),
            end_of_lock: Instant::now(),
            game_ended: false,
            fall_rate: Duration::from_millis(1000),
        };

        screen.spawn_player();

        screen
    }

    pub fn eat_player(&mut self) {
        self.opponent.merge(self.player.clone());
        let lines_removed = self.opponent.remove_full_lines();
    }

    pub fn spawn_player(&mut self) {
        let mut player = self.next_player.clone();
        player.row -= player.height as i8;
        player.column = (PUZZLE_WIDTH - player.width) as i8 / 2;

        self.player = player;
        self.next_player = random_tetromino();
    }

    pub fn rotate_player(&mut self) {
        let mut foreshadow = self.player.clone();
        foreshadow.rotate();
        if foreshadow.collides_with(&self.opponent) || !foreshadow.within_bounds() {
            return;
        }
        self.player = foreshadow
    }

    pub fn move_player(&mut self, row_direction: u8, column_direction: i8) -> bool {
        let mut foreshadow = self.player.clone();
        let moving_down = row_direction == 1;
        foreshadow.translate(row_direction, column_direction);
        let able_to_move = !foreshadow.collides_with(&self.opponent) && foreshadow.within_bounds();

        if able_to_move {
            self.player = foreshadow;
            if moving_down {
                self.on_floor = false
            }
        } else if moving_down {
            self.handle_falling_down();
        }
        able_to_move
    }

    pub fn handle_falling_down(&mut self) {
        let time = Instant::now();
        if !self.on_floor {
            self.on_floor = true;
            self.end_of_lock = time + self.floor_rate;
            return;
        } else if time < self.end_of_lock {
            return;
        }

        self.eat_player();
        self.spawn_player();

        if self.player.collides_with(&self.opponent) {
            self.game_ended = true
        }
    }

    pub fn move_right(&mut self) {
        self.move_player(0, 1);
    }

    pub fn move_left(&mut self) {
        self.move_player(0, -1);
    }

    pub fn fall_down(&mut self) -> bool {
        let able_to_move = self.move_player(1, 0);

        able_to_move
    }

    pub fn is_game_ended(&self) -> bool {
        self.game_ended
    }
}
