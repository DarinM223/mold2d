use cache;
use sdl2::SdlResult;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2_ttf;
use sdl2_ttf::Font;
use sprite::{Renderable, Sprite};
use std::path::Path;

/// Returns a text sprite with the specified text, font, size, and color
pub fn text_sprite(renderer: &Renderer,
                   text: &str,
                   font_path: &'static str,
                   size: i32,
                   color: Color)
                   -> SdlResult<Sprite> {
    let font_cache = cache::font_cache();

    // if font is cached use the cached font
    {
        if let Ok(ref cache) = font_cache.cache.lock() {
            if let Some(font) = cache.get(font_path) {
                let surface = try!(font.render(text, sdl2_ttf::blended(color)));
                let texture = try!(renderer.create_texture_from_surface(&surface));

                return Ok(Sprite::new(texture));
            }
        }
    }

    // otherwise load font from file path
    let font = try!(Font::from_file(Path::new(font_path), size));
    let sprite;

    {
        let surface = try!(font.render(text, sdl2_ttf::blended(color)));
        let texture = try!(renderer.create_texture_from_surface(&surface));

        sprite = Sprite::new(texture);
    }

    // cache if successful
    if let Ok(ref mut cache) = font_cache.cache.lock() {
        cache.insert(font_path.to_owned(), font);
    }

    Ok(sprite)
}

/// Renders a text sprite at the specified point
pub fn render_text(renderer: &mut Renderer, sprite: &Sprite, point: (i32, i32)) {
    let (x, y) = point;
    let (w, h) = sprite.size();
    sprite.render(renderer, Rect::new_unwrap(x, y, w, h));
}
