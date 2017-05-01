use cache;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::ttf;
use sdl2::ttf::Font;
use sprite::{Renderable, Sprite};
use std::error::Error;
use std::path::Path;

/// Returns a text sprite with the specified text, font, size, and color
pub fn text_sprite(renderer: &Renderer,
                   text: &str,
                   font_path: &'static str,
                   size: i32,
                   color: Color)
                   -> Result<Sprite, Box<Error>> {
    let font_cache = cache::font_cache();

    // if font is cached use the cached font
    {
        if let Ok(ref cache) = font_cache.cache.lock() {
            if let Some(font) = cache.get(font_path) {
                let surface = try!(font.render(text).blended(color));
                let texture = try!(renderer.create_texture_from_surface(&surface));

                return Ok(Sprite::new(texture));
            }
        }
    }

    // otherwise load font from file path
    let font = try!(cache::GlobalTtfContext.load_font(Path::new(font_path), 12));
    let sprite;

    {
        let surface = try!(font.render(text).blended(color));
        let texture = try!(renderer.create_texture_from_surface(&surface));

        sprite = Sprite::new(texture);
    }

    // cache if successful
    let _ = font_cache
        .cache
        .lock()
        .map(|ref mut cache| cache.insert(font_path.to_owned(), font));

    Ok(sprite)
}

/// Renders a text sprite at the specified point
pub fn render_text(renderer: &mut Renderer, sprite: &Sprite, point: (i32, i32)) {
    let (x, y) = point;
    let (w, h) = sprite.size();
    sprite.render(renderer, Rect::new(x, y, w, h));
}
