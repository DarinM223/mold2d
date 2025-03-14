use crate::cache;
use crate::collision;
use crate::collision::{BoundingBox, Collision, CollisionSide};
use crate::vector::PositionChange;
use crate::viewport::Viewport;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::path::Path;
use std::rc::Rc;

/// The direction that a sprite is facing
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    Left,
    Right,
}

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas<Window>, dest: Rect) -> Result<(), Box<dyn Error>>;
}

/// A mutable rectangle for a sprite so it can be moved around
#[derive(Clone, Copy, PartialEq)]
pub struct SpriteRectangle {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl SpriteRectangle {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> SpriteRectangle {
        SpriteRectangle { x, y, w, h }
    }

    /// Creates a sprite rectangle from a SDL2 rectangle
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
    pub fn to_sdl(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }

    /// Mutates a sprite rectangle based on the position change given
    pub fn apply_change(&mut self, change: &PositionChange) {
        self.x += change.x;
        self.y += change.y;
        if change.w < 0 {
            self.w -= change.w.unsigned_abs();
        } else {
            self.w += change.w as u32;
        }
        if change.h < 0 {
            self.h -= change.h.unsigned_abs();
        } else {
            self.w += change.w as u32;
        }
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
            src: Rect::new(0, 0, tex_query.width, tex_query.height),
        }
    }

    /// Loads a new sprite from a path string to a sprite image file
    pub fn load(canvas: &Canvas<Window>, path: &str) -> Result<Sprite, Box<dyn Error>> {
        let sprite_cache = cache::sprite_cache();

        // if sprite is cached, return from cache
        if let Ok(ref cache) = sprite_cache.cache.lock() {
            if let Some(sprite) = cache.get(path).cloned() {
                return Ok(sprite);
            }
        }

        // otherwise load sprite from texture
        let creator = canvas.texture_creator();
        let sprite = creator.load_texture(Path::new(path)).map(Sprite::new);

        // cache result if successful
        let _ = sprite.clone().map(|sprite| {
            sprite_cache
                .cache
                .lock()
                .map(|ref mut cache| cache.insert(path.to_owned(), sprite))
        });

        sprite.map_err(From::from)
    }

    /// Returns a sub-sprite from a rectangle region of the original sprite
    pub fn region(&self, rect: Rect) -> Option<Sprite> {
        let new_src = Rect::new(
            rect.x() + self.src.x(),
            rect.y() + self.src.y(),
            rect.width(),
            rect.height(),
        );

        if collision::rect_contains_rect(self.src, new_src) {
            Some(Sprite {
                tex: self.tex.clone(),
                src: new_src,
            })
        } else {
            None
        }
    }

    /// Returns the dimensions of the sprite
    pub fn size(&self) -> (u32, u32) {
        (self.src.width(), self.src.height())
    }
}

impl Renderable for Sprite {
    /// Render the sprite image onto the rectangle
    fn render(&self, canvas: &mut Canvas<Window>, dest: Rect) -> Result<(), Box<dyn Error>> {
        canvas
            .copy(&self.tex.borrow_mut(), Some(self.src), Some(dest))
            .map_err(From::from)
    }
}

/// Represents an animated sprite with multiple frames
#[derive(Clone)]
pub struct AnimatedSprite {
    /// Frames that will be rendered
    frames: Vec<Sprite>,
    /// Time between frames
    frame_delay: f64,
    /// Total time sprite has been alive
    current_time: f64,
}

impl AnimatedSprite {
    /// Creates a new animated sprite with the given Sprite frames and a frame delay
    fn new(frames: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        AnimatedSprite {
            frames,
            frame_delay,
            current_time: 0.0,
        }
    }

    pub fn with_fps(frames: Vec<Sprite>, fps: f64) -> AnimatedSprite {
        assert!(fps != 0.0);
        AnimatedSprite::new(frames, 1.0 / fps)
    }

    fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }

    fn set_fps(&mut self, fps: f64) {
        assert!(fps != 0.0);
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
    /// Renders the current frame of the animated sprite
    fn render(&self, canvas: &mut Canvas<Window>, dest: Rect) -> Result<(), Box<dyn Error>> {
        assert!(
            !self.frames.is_empty(),
            "There as to be at least one frame!"
        );
        let current_frame = (self.current_time / self.frame_delay) as usize % self.frames.len();

        let frame = &self.frames[current_frame];
        frame.render(canvas, dest)
    }
}

/// Contains configuration fields for parsing a spritesheet
pub struct SpritesheetConfig {
    /// The width of each animation frame
    pub width: u32,
    /// The height of each animation frame
    pub height: u32,
    /// The number of frames in each row of the spritesheet
    pub sprites_in_row: i32,
    /// The path to the spritesheet file
    pub path: &'static str,
}

/// A spritesheet manager that returns sprites within a range
pub struct Spritesheet {
    config: SpritesheetConfig,
    spritesheet: Sprite,
}

impl Spritesheet {
    /// Loads a spritesheet given a configuration object and a SDL2 canvas
    pub fn new(config: SpritesheetConfig, canvas: &mut Canvas<Window>) -> Spritesheet {
        let spritesheet = Sprite::load(canvas, config.path).unwrap();

        Spritesheet {
            config,
            spritesheet,
        }
    }

    /// Returns a Vec of sprites within a certain range.
    /// The number of the start and end ranges are from row to
    /// row wrapping around. For example a 3x3 grid would have
    /// indexes like this:
    /// ```
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    pub fn range(&self, start: i32, end: i32) -> Vec<Sprite> {
        (start..end)
            .flat_map(|elem| {
                let x = elem % self.config.sprites_in_row;
                let y = elem / self.config.sprites_in_row;

                let region = Rect::new(
                    (self.config.width as i32) * x,
                    (self.config.height as i32) * y,
                    self.config.width,
                    self.config.height,
                );
                self.spritesheet.region(region)
            })
            .collect()
    }
}

/// An animation manager that retrieves the animations for a state
pub struct Animations<State> {
    fps: f64,
    animations: HashMap<State, (AnimatedSprite, BoundingBox)>,
    /// Saves the current state for better performance
    curr_state: Option<State>,
    /// Saves the current bounding box for better performance
    curr_bbox: Option<BoundingBox>,
    /// Saves the current animation for better performance
    curr_anim: Option<AnimatedSprite>,
}

impl<State> Animations<State>
where
    State: Clone + Eq + Hash,
{
    pub fn new(fps: f64) -> Animations<State> {
        Animations {
            fps,
            animations: HashMap::new(),
            curr_state: None,
            curr_bbox: None,
            curr_anim: None,
        }
    }

    pub fn add(&mut self, s: State, anims: Vec<Sprite>, bound: BoundingBox) {
        self.animations
            .insert(s, (AnimatedSprite::with_fps(anims, self.fps), bound));
    }

    fn set_state(&mut self, s: &State) {
        // Insert the saved bounding box and animation back into the hashmap
        if let (Some(state), Some(bbox), Some(anim)) = (
            self.curr_state.take(),
            self.curr_bbox.take(),
            self.curr_anim.take(),
        ) {
            self.animations.insert(state, (anim, bbox));
        }

        // Save the new state
        self.curr_state = Some(s.clone());
        if let Some((anim, bbox)) = self.animations.remove(s) {
            self.curr_bbox = Some(bbox);
            self.curr_anim = Some(anim);
        }
    }

    /// Returns an immutable reference to the animation for the given state
    pub fn anim(&mut self, s: &State) -> Option<&AnimatedSprite> {
        if let Some(ref state) = self.curr_state {
            if state == s {
                return self.curr_anim.as_ref();
            }
        }

        self.set_state(s);
        self.curr_anim.as_ref()
    }

    /// Returns a mutable reference to the animation for the given state
    pub fn anim_mut(&mut self, s: &State) -> Option<&mut AnimatedSprite> {
        if let Some(ref state) = self.curr_state {
            if state == s {
                return self.curr_anim.as_mut();
            }
        }

        self.set_state(s);
        self.curr_anim.as_mut()
    }

    /// Returns an immutable reference to the bounding box for the given state
    pub fn bbox(&mut self, s: &State) -> Option<&BoundingBox> {
        if let Some(ref state) = self.curr_state {
            if state == s {
                return self.curr_bbox.as_ref();
            }
        }

        self.set_state(s);
        self.curr_bbox.as_ref()
    }

    /// Returns a mutable reference to the bounding box for the given state
    pub fn bbox_mut(&mut self, s: &State) -> Option<&mut BoundingBox> {
        if let Some(ref state) = self.curr_state {
            if state == s {
                return self.curr_bbox.as_mut();
            }
        }

        self.set_state(s);
        self.curr_bbox.as_mut()
    }

    /// Maps a function that mutates a bounding box over all of the
    /// bounding boxes in the animation
    pub fn map_bbox_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut BoundingBox),
    {
        if let Some(ref mut bbox) = self.curr_bbox {
            f(bbox);
        }

        for animation in self.animations.iter_mut() {
            let bbox = &mut (animation.1).1;
            f(bbox);
        }
    }

    /// Checks if the animation at the state collides with another bounding box
    /// and returns the side of the collision if it happens
    pub fn collides_with(
        &mut self,
        s: &State,
        other_bbox: &Option<BoundingBox>,
    ) -> Option<CollisionSide> {
        if let Some(bounding_box) = self.bbox(s) {
            if let Some(ref bbox) = *other_bbox {
                return bounding_box.collides_with(bbox);
            }
        }

        None
    }

    /// Adds time to the current animation
    pub fn add_time(&mut self, s: &State, elapsed: f64) {
        let _ = self.anim_mut(s).map(|ref mut anim| anim.add_time(elapsed));
    }

    /// Renders an animation in the manager
    pub fn render(
        &mut self,
        s: &State,
        rect: &SpriteRectangle,
        viewport: &mut Viewport,
        canvas: &mut Canvas<Window>,
        debug: bool,
    ) -> Result<(), Box<dyn Error>> {
        if debug {
            if let Some(bounding_box) = self.bbox(s) {
                match *bounding_box {
                    BoundingBox::Rectangle(ref rect) => {
                        canvas.set_draw_color(::sdl2::pixels::Color::RGB(230, 230, 230));
                        let (rx, ry) = viewport.relative_point((rect.x, rect.y));
                        let rect = Rect::new(rx, ry, rect.w, rect.h);
                        canvas.fill_rect(rect)?;
                    }
                }
            }
        }

        let (rx, ry) = viewport.relative_point((rect.x, rect.y));
        let rect = Rect::new(rx, ry, rect.w, rect.h);

        self.anim_mut(s).unwrap().render(canvas, rect)
    }
}
