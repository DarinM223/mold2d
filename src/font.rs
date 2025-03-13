use crate::cache;
use crate::sprite::{Renderable, Sprite};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::error::Error;
use std::path::Path;

/// Returns a text sprite with the specified text, font, size, and color
pub fn text_sprite(
    canvas: &Canvas<Window>,
    text: &str,
    font_path: &'static str,
    size: u16,
    color: Color,
) -> Result<Sprite, Box<dyn Error>> {
    let font_cache = cache::font_cache();
    let creator = canvas.texture_creator();

    // if font is cached use the cached font
    if let Ok(ref cache) = font_cache.cache.lock() {
        if let Some(font) = cache.get(font_path) {
            let surface = font.render(text).blended(color)?;
            let texture = creator.create_texture_from_surface(&surface)?;

            return Ok(Sprite::new(texture));
        }
    }

    // otherwise load font from file path
    let font = cache::TTF_CONTEXT.load_font(Path::new(font_path), size)?;

    let surface = font.render(text).blended(color)?;
    let texture = creator.create_texture_from_surface(&surface)?;

    let sprite = Sprite::new(texture);

    // cache if successful
    let _ = font_cache
        .cache
        .lock()
        .map(|ref mut cache| cache.insert(font_path.to_owned(), font));

    Ok(sprite)
}

/// Renders a text sprite at the specified point
pub fn render_text(
    canvas: &mut Canvas<Window>,
    sprite: &Sprite,
    point: (i32, i32),
) -> Result<(), Box<dyn Error>> {
    let (x, y) = point;
    let (w, h) = sprite.size();
    sprite.render(canvas, Rect::new(x, y, w, h))
}
