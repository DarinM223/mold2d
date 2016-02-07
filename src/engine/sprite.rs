use engine::physics::collision;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use sdl2_image::LoadTexture;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: Rect);
}

/// A mutable rectangle for a sprite so it can be moved around
pub struct SpriteRectangle {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl SpriteRectangle {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> SpriteRectangle {
        SpriteRectangle {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    pub fn from_rect(rect: Rect) -> SpriteRectangle {
        SpriteRectangle {
            x: rect.x(),
            y: rect.y(),
            w: rect.width(),
            h: rect.height(),
        }
    }

    /// Returns a SDL Rect created from the SpriteRectangle
    /// Used for rendering SpriteRectangles in SDL
    pub fn to_sdl(&self) -> Option<Rect> {
        Rect::new(self.x, self.y, self.w, self.h).unwrap()
    }
}

/// A sprite data type that uses reference counting
/// to reuse the texture on multiple sub-sprites
#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rect,
}

impl Sprite {
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rect::new_unwrap(0, 0, tex_query.width, tex_query.height),
        }
    }

    /// Loads a new sprite from a path string to a sprite image file
    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    /// Returns a sub-sprite from a rectangle region of the original sprite 
    pub fn region(&self, rect: Rect) -> Option<Sprite> {
        let new_src = Rect::new_unwrap(rect.x() + self.src.x(),
                                       rect.y() + self.src.y(),
                                       rect.width(),
                                       rect.height());

        if collision::rect_contains_rect(self.src, new_src) {
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

/// Represents an animated sprite with multiple frames
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

    pub fn with_fps(frames: Vec<Sprite>, fps: f64) -> AnimatedSprite {
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

    /// Updates the animated sprite with the elapsed time
    pub fn add_time(&mut self, elapsed: f64) {
        self.current_time += elapsed;

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
///     sprite_side: 100,
///     sprites_in_row: 5,
///     animations: {
///        Idle: 0..5,
///        Walking: 5..10,
///        Running: 10..15
///     },
///     properties: {
///        curr_state: KoopaState => KoopaState::Idle,
///        vel: f64 => 0.0
///     }
/// }
/// ```
macro_rules! spritesheet {
    (
        name: $name:ident,
        state: $state:ident,
        path: $path:expr,
        sprite_side: $sprite_side:expr,
        sprites_in_row: $sprites_in_row:expr,
        animations: { $( $a_alias:ident : $a_range:expr ),* },
        properties: { $( $p_alias:ident : $p_type:ident => $p_value:expr ),* }
    ) => {
        #[derive(Clone, PartialEq, Eq, Debug, Hash)]
        pub enum $state {
            $( $a_alias ),*
        }

        pub struct $name {
            pub path: &'static str,
            pub animations: ::std::collections::HashMap<$state, ::engine::sprite::AnimatedSprite>,
            $( pub $p_alias: $p_type ),*
        }

        impl $name {
            pub fn new(renderer: &mut ::sdl2::render::Renderer, fps: f64) -> $name {
                let spritesheet = match ::engine::sprite::Sprite::load(renderer, $path) {
                    Some(spritesheet) => spritesheet,
                    None => panic!("{} is not a valid path", $path),
                };

                let mut animations = ::std::collections::HashMap::new();

                $(
                    let sprites: Vec<::engine::sprite::Sprite> = $a_range.map(|elem| {
                        let x = elem % $sprites_in_row;
                        let y = elem / $sprites_in_row;

                        let region = ::sdl2::rect::Rect::new_unwrap($sprite_side * x, $sprite_side * y, $sprite_side, $sprite_side);

                        spritesheet.region(region)
                    }).flat_map(|sprite| sprite).collect();

                    animations.insert($state::$a_alias, ::engine::sprite::AnimatedSprite::with_fps(sprites, fps));
                 )*

                 $name {
                     path: $path,
                     animations: animations,
                     $( $p_alias: $p_value ),*
                 }
            }
        }
    }
}
