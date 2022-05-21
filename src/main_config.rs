// Game Screen constants
pub const SQUARE_WIDTH: i32 = 24;
pub const SQUARE_BORDER_WIDTH: i32 = 3;
pub const PUZZLE_HEIGHT: i32 = 20;
pub const PUZZLE_WIDTH: i32 = 10;
pub const SIDEBAR_WIDTH: i32 = SQUARE_WIDTH * 6;
pub const WAR_ZONE_WIDTH: i32 = PUZZLE_WIDTH * SQUARE_WIDTH;

// Window Constants
pub const CANVAS_WIDTH: i32 = SIDEBAR_WIDTH + WAR_ZONE_WIDTH;
pub const CANVAS_HEIGHT: i32 = PUZZLE_HEIGHT * SQUARE_WIDTH;
// pub const GAME_ID: &'static str = "rustris";
// pub const GAME_AUTHOR: &'static str = "Abderrahmane Tahri Jouti";
pub const WINDOW_TITLE: &'static str = "Rustris";
