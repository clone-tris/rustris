use crate::screens::game_screen::config::{
    PUZZLE_HEIGHT, SIDEBAR_WIDTH, SQUARE_WIDTH, WAR_ZONE_WIDTH,
};

pub const CANVAS_WIDTH: u16 = SIDEBAR_WIDTH + WAR_ZONE_WIDTH;
pub const CANVAS_HEIGHT: u16 = PUZZLE_HEIGHT * SQUARE_WIDTH;
pub const GAME_ID: &'static str = "rustris";
pub const GAME_AUTHOR: &'static str = "Abderrahmane Tahri Jouti";
pub const WINDOW_TITLE: &'static str = "Clonetris, made easy, in Rust!";
