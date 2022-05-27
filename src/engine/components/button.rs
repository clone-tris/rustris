use sdl2::rect::Point;
use sdl2::ttf::Font;

struct Button<'a> {
    text: String,
    position: Point,
    font: Font<'a, 'a>,
}
