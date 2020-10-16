pub type RGB = (u8, u8, u8);
pub type RGBA = (u8, u8, u8, u8);

pub mod ui_colors {
    use crate::screens::game_screen::colors::RGB;

    pub const BACKGROUND: RGB = (0x33, 0x33, 0x33);
    pub const SIDEBAR_BACKGROUND: RGB = (0x54, 0x54, 0x54);
    pub const POPUP_BACKGROUND: RGB = (0x21, 0x21, 0x21);
    pub const GUIDE: RGB = (0x55, 0x55, 0x55);
    pub const WHITE_TEXT: RGB = (0xFF, 0xFF, 0xFF);
    pub const POPUP_TEXT: RGB = (0xEF, 0xEF, 0xEF);
}

pub mod shape_colors {
    use crate::screens::game_screen::colors::{RGB, RGBA};

    pub const CYAN: RGB = (0x6D, 0xEC, 0xEE);
    pub const BLUE: RGB = (0x00, 0x14, 0xE6);
    pub const ORANGE: RGB = (0xE4, 0xA3, 0x38);
    pub const YELLOW: RGB = (0xF0, 0xEF, 0x4F);
    pub const GREEN: RGB = (0x6E, 0xEB, 0x47);
    pub const PURPLE: RGB = (0x92, 0x25, 0xE7);
    pub const RED: RGB = (0xDC, 0x2F, 0x20);
    pub const DEFAULT_SQUARE_COLOR: RGB = (0xCC, 0x80, 0x81);
    pub const BORDER_TOP: RGBA = (0xFF, 0xFF, 0xFF, 178);
    pub const BORDER_BOTTOM: RGBA = (0, 0, 0, 127);
    pub const BORDER_SIDE: RGBA = (0, 0, 0, 25);
}
