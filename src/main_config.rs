// Game Screen constants
pub const SQUARE_WIDTH: i16 = 24;
pub const SQUARE_BORDER_WIDTH: i16 = 3;
pub const PUZZLE_HEIGHT: i16 = 20;
pub const PUZZLE_WIDTH: i16 = 10;
pub const SIDEBAR_WIDTH: i16 = SQUARE_WIDTH * 6;
pub const WAR_ZONE_WIDTH: i16 = PUZZLE_WIDTH as i16 * SQUARE_WIDTH;

// Window Constants
pub const CANVAS_WIDTH: i16 = SIDEBAR_WIDTH + WAR_ZONE_WIDTH;
pub const CANVAS_HEIGHT: i16 = PUZZLE_HEIGHT * SQUARE_WIDTH;
pub const GAME_ID: &'static str = "rustris";
pub const GAME_AUTHOR: &'static str = "Abderrahmane Tahri Jouti";
pub const WINDOW_TITLE: &'static str = "Rustris";
