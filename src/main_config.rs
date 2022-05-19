// Game Screen constants
pub const SQUARE_WIDTH: u32 = 24;
pub const SQUARE_BORDER_WIDTH: u32 = 3;
pub const PUZZLE_HEIGHT: u32 = 20;
pub const PUZZLE_WIDTH: u32 = 10;
pub const SIDEBAR_WIDTH: u32 = SQUARE_WIDTH * 6;
pub const WAR_ZONE_WIDTH: u32 = PUZZLE_WIDTH as u32 * SQUARE_WIDTH;

// Window Constants
pub const CANVAS_WIDTH: u32 = SIDEBAR_WIDTH + WAR_ZONE_WIDTH;
pub const CANVAS_HEIGHT: u32 = PUZZLE_HEIGHT * SQUARE_WIDTH;
pub const GAME_ID: &'static str = "rustris";
pub const GAME_AUTHOR: &'static str = "Abderrahmane Tahri Jouti";
pub const WINDOW_TITLE: &'static str = "Rustris";
