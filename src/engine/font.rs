use sdl2::SdlResult;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2_ttf;
use sdl2_ttf::Font;
use sprite::{Renderable, Sprite};
use std::collections::HashMap;
use std::path::Path;

pub struct FontRenderer {
    fonts: HashMap<String, Font>,
}

impl FontRenderer {
    pub fn new() -> FontRenderer {
        FontRenderer { fonts: HashMap::new() }
    }

    pub fn text_sprite(&mut self,
                       renderer: &Renderer,
                       text: &str,
                       font_path: &'static str,
                       size: i32,
                       color: Color)
                       -> SdlResult<Sprite> {
        {
            if let Some(font) = self.fonts.get(font_path) {
                let surface = try!(font.render(text, sdl2_ttf::blended(color)));
                let texture = try!(renderer.create_texture_from_surface(&surface));

                return Ok(Sprite::new(texture));
            }
        }

        let font = try!(Font::from_file(Path::new(font_path), size));
        let sprite;

        {
            let surface = try!(font.render(text, sdl2_ttf::blended(color)));
            let texture = try!(renderer.create_texture_from_surface(&surface));

            sprite = Sprite::new(texture);
        }

        self.fonts.insert(font_path.to_owned(), font);

        Ok(sprite)
    }

    pub fn render_text(&mut self, renderer: &mut Renderer, sprite: Sprite, point: (i32, i32)) {
        let (x, y) = point;
        let (w, h) = sprite.size();
        sprite.render(renderer, Rect::new_unwrap(x, y, w, h));
    }
}
