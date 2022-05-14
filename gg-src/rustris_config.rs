use crate::screens::game_screen::config::{
    PUZZLE_HEIGHT, SIDEBAR_WIDTH, SQUARE_WIDTH, WAR_ZONE_WIDTH,
};

pub const CANVAS_WIDTH: i16 = SIDEBAR_WIDTH + WAR_ZONE_WIDTH;
pub const CANVAS_HEIGHT: i16 = PUZZLE_HEIGHT * SQUARE_WIDTH;
pub const GAME_ID: &'static str = "rustris";
pub const GAME_AUTHOR: &'static str = "Abderrahmane Tahri Jouti";
pub const WINDOW_TITLE: &'static str = "Rustris";
