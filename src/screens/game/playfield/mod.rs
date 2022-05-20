mod painter;

use crate::engine::screen::Screen;

use crate::screens::game::playfield::painter::Painter;

use crate::colors::ShapeColors;
use crate::main_config::{PUZZLE_HEIGHT, PUZZLE_WIDTH};
use crate::screens::game::components::shape::Shape;
use crate::screens::game::components::tetromino::random_tetromino;
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};

pub struct PlayField {
    painter: Painter,
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

impl Screen for PlayField {
    fn paint(&mut self, canvas: &mut WindowCanvas) {
        self.painter.setup(canvas);
        self.painter.background(canvas);
        self.painter.draw_guide(canvas);
        self.player.draw(canvas);
        self.opponent.draw(canvas);
    }
}

impl PlayField {
    pub fn new(width: i32, height: i32) -> PlayField {
        let mut opponent = Shape::new(Vec::new(), 0, 0, ShapeColors::DefaultSquareColor.value());
        opponent.width = PUZZLE_WIDTH;
        opponent.height = PUZZLE_HEIGHT;

        let mut screen = PlayField {
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
        let _lines_removed = self.opponent.remove_full_lines();
    }

    pub fn spawn_player(&mut self) {
        let mut player = self.next_player.clone();
        player.row -= player.height;
        player.column = (PUZZLE_WIDTH - player.width) / 2;

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

    pub fn move_player(&mut self, row_direction: i32, column_direction: i32) -> bool {
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
