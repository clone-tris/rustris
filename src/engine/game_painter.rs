use crate::colors::{ShapeColors, UiColors};
use crate::main_config::SQUARE_WIDTH;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub fn set_viewport(canvas: &mut WindowCanvas, x: i32, y: i32, width: i32, height: i32) {
    canvas.set_viewport(Rect::new(x, y, width as u32, height as u32));
}

pub fn draw_background(canvas: &mut WindowCanvas, width: i32, height: i32, color: Color) {
    canvas.set_draw_color(color);
    canvas
        .fill_rect(Rect::new(0, 0, width as u32, height as u32))
        .unwrap();
}

pub fn draw_line(canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    let (origin, destination) = (Point::new(x1, y1), Point::new(x2, y2));
    canvas.set_draw_color(color);
    canvas.draw_line(origin, destination).unwrap();
}

pub fn draw_guide(canvas: &mut WindowCanvas, x: i32, y: i32, width: i32, height: i32) {
    let rows = height / SQUARE_WIDTH;
    let columns = width / SQUARE_WIDTH;

    for i in 0..(rows + 1) {
        let line_y = y + i * SQUARE_WIDTH;
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
        let line_x = x + i * SQUARE_WIDTH;
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

pub fn draw_text<'t>(
    canvas: &mut WindowCanvas,
    font: &Font,
    texture_creator: &'t TextureCreator<WindowContext>,
    at: Point,
    text: String,
    color: Color,
) -> Texture<'t> {
    let surface = font.render(text.as_str()).blended(color).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();
    canvas
        .copy(&texture, None, Rect::new(at.x, at.y, width, height))
        .unwrap();

    texture
}

pub fn draw_button(
    canvas: &mut WindowCanvas,
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    at: Point,
    text: String,
) {
    let texture = draw_text(
        canvas,
        font,
        texture_creator,
        at,
        text,
        UiColors::ButtonText.value(),
    );
}
