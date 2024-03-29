// #![allow(dead_code)]
// #![allow(unused_variables)]

use sdl2::pixels::Color;

pub type RGBA = (u8, u8, u8, u8);

// Ui
pub const BACKGROUND: RGBA = (0x33, 0x33, 0x33, 0xFF);
pub const SIDEBAR_BACKGROUND: RGBA = (0x44, 0x44, 0x44, 0xFF);
pub const POPUP_BACKGROUND: RGBA = (0x21, 0x21, 0x21, 0xFF);
pub const GUIDE: RGBA = (0x65, 0x65, 0x65, 0xFF);
pub const LIGHT_TEXT: RGBA = (0xFF, 0xFF, 0xFF, 0xFF);
pub const DARK_TEXT: RGBA = (0x21, 0x21, 0x21, 0xFF);
pub const POPUP_TEXT: RGBA = (0xEF, 0xEF, 0xEF, 0xFF);

// Shapes
pub const CYAN: RGBA = (0x6D, 0xEC, 0xEE, 0xFF);
pub const BLUE: RGBA = (0x00, 0x14, 0xE6, 0xFF);
pub const ORANGE: RGBA = (0xE4, 0xA3, 0x38, 0xFF);
pub const YELLOW: RGBA = (0xF0, 0xEF, 0x4F, 0xFF);
pub const GREEN: RGBA = (0x6E, 0xEB, 0x47, 0xFF);
pub const PURPLE: RGBA = (0x92, 0x25, 0xE7, 0xFF);
pub const RED: RGBA = (0xDC, 0x2F, 0x20, 0xFF);
pub const DEFAULT_SQUARE_COLOR: RGBA = (0xCC, 0x80, 0x81, 0xFF);
pub const BORDER_TOP: RGBA = (0xFF, 0xFF, 0xFF, 178);
pub const BORDER_BOTTOM: RGBA = (0, 0, 0, 127);
pub const BORDER_SIDE: RGBA = (0, 0, 0, 25);

pub enum UiColors {
    Background,
    SidebarBackground,
    PopupBackground,
    Guide,
    WhiteText,
    PopupText,
    ButtonText,
    ButtonBackground,
}

impl UiColors {
    pub fn value(&self) -> Color {
        match *self {
            UiColors::Background => Color::from(BACKGROUND),
            UiColors::SidebarBackground => Color::from(SIDEBAR_BACKGROUND),
            UiColors::PopupBackground => Color::from(POPUP_BACKGROUND),
            UiColors::ButtonBackground => Color::from(CYAN),
            UiColors::Guide => Color::from(GUIDE),
            UiColors::WhiteText => Color::from(LIGHT_TEXT),
            UiColors::PopupText => Color::from(POPUP_TEXT),
            UiColors::ButtonText => Color::from(DARK_TEXT),
        }
    }
}

pub enum ShapeColors {
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
    DefaultSquareColor,
    BorderTop,
    BorderBottom,
    BorderSide,
}

impl ShapeColors {
    pub fn value(&self) -> Color {
        match *self {
            ShapeColors::Cyan => Color::from(CYAN),
            ShapeColors::Blue => Color::from(BLUE),
            ShapeColors::Orange => Color::from(ORANGE),
            ShapeColors::Yellow => Color::from(YELLOW),
            ShapeColors::Green => Color::from(GREEN),
            ShapeColors::Purple => Color::from(PURPLE),
            ShapeColors::Red => Color::from(RED),
            ShapeColors::DefaultSquareColor => Color::from(DEFAULT_SQUARE_COLOR),
            ShapeColors::BorderTop => Color::from(BORDER_TOP),
            ShapeColors::BorderBottom => Color::from(BORDER_BOTTOM),
            ShapeColors::BorderSide => Color::from(BORDER_SIDE),
        }
    }
}
