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

impl Renderable for Sprite {
    fn render(&self, renderer: &mut Renderer, dest: Rect) {
        renderer.copy(&mut self.tex.borrow_mut(), Some(self.src), Some(dest));
    }
}

pub struct AnimatedSprite {
    /// frames that will be rendered
    frames: Vec<Sprite>,

    /// time between frames
    frame_delay: f64,

    /// total time sprite has been alive
    current_time: f64,
}

impl AnimatedSprite {
    fn new(frames: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        AnimatedSprite {
            frames: frames,
            frame_delay: frame_delay,
            current_time: 0.0,
        }
    }

    fn with_fps(frames: Vec<Sprite>, fps: f64) -> AnimatedSprite {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::with_fps");
        }

        AnimatedSprite::new(frames, 1.0 / fps)
    }

    fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    fn set_fps(&mut self, fps: f64) {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::set_fps");
        }

        self.set_frame_delay(1.0 / fps);
    }

    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;

        if self.current_time < 0.0 {
            self.current_time = (self.frames.len() - 1) as f64 * self.frame_delay;
        }
    }
}

impl Renderable for AnimatedSprite {
    fn render(&self, renderer: &mut Renderer, dest: Rect) {
        let current_frame = (self.current_time / self.frame_delay) as usize % self.frames.len();

        let frame = &self.frames[current_frame];
        frame.render(renderer, dest);
    }
}

/// Generates a spritesheet actor
/// Example:
/// ```
/// spritesheet! {
///     name: Koopa,
///     state: KoopaState,
///     path: "/assets/foo",
///     animations: {
///        Idle: 1..5
///        Walking: 5..10,
///        Running: 10..15,
///     }
/// }
/// ```
///
/// should generate:
///
/// ```
/// use engine::sprite::AnimatedSprite;
///
/// #[derive(Clone, PartialEq, Debug)]
/// pub enum KoopaState {
///     Idle,
///     Walking,
///     Running,
/// }
///
/// #[derive(Clone, Debug)]
/// pub struct Koopa {
///     path: "/assets/foo",
///     Idle: AnimatedSprite,
///     Walking: AnimatedSprite,
///     Running: AnimatedSprite,
/// }
/// ```
macro_rules! spritesheet {
    (
        name: $name:ident,
        state: $state:ident,
        path: $path:expr,
        animations: { $( $a_alias:ident : $a_range:expr ),* }
    ) => {
        use engine::sprite::AnimatedSprite;

        #[derive(Clone, PartialEq, Debug)]
        pub enum $state {
            $( $a_alias ),*
        }

        pub struct $name {
            pub path: String,
            $( pub $a_alias: AnimatedSprite ),*
        }
    }
}
