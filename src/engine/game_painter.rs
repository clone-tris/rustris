use crate::colors;
use crate::colors::UiColors;
use crate::main_config::SQUARE_WIDTH;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub fn draw_line(canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    let (origin, destination) = (Point::new(x1, y1), Point::new(x2, y2));
    canvas.set_draw_color(color);
    canvas.draw_line(origin, destination);
}

pub fn draw_guide(canvas: &mut WindowCanvas, x: i32, y: i32, width: i32, height: i32) {
    let rows = (height / SQUARE_WIDTH as i32);
    let columns = (width / SQUARE_WIDTH as i32);

    for i in 0..(rows + 1) {
        let line_y = y + i * SQUARE_WIDTH as i32;
        draw_line(
            canvas,
            x,
            line_y,
            x + width,
            line_y,
            UiColors::Guide.value(),
        );
    }
    for i in 0..(columns + 1) {
        let line_x = x + i * SQUARE_WIDTH as i32;
        draw_line(
            canvas,
            line_x,
            y,
            line_x,
            y + height,
            UiColors::Guide.value(),
        );
    }
}
