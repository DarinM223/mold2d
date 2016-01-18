use engine::geo_utils::{GeoUtils, Shape};
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: Rect);
}

#[derive(Clone)]
struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rect,
}

impl Sprite {
    fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rect::new(0, 0, tex_query.width, tex_query.height).unwrap().unwrap(),
        }
    }

    fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    fn region(&self, rect: Rect) -> Option<Sprite> {
        let new_src = Rect::new(rect.x() + self.src.x(),
                                rect.y() + self.src.y(),
                                rect.width(),
                                rect.height())
                          .unwrap()
                          .unwrap();

        if GeoUtils::contains(self.src, new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src,
            })
        } else {
            None
        }
    }
}
