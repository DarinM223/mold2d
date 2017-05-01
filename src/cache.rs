use sdl2::ttf::{Font, Sdl2TtfContext};
use sprite::Sprite;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex, ONCE_INIT, Once};

pub static GlobalTtfContext: Sdl2TtfContext = Sdl2TtfContext;

/// A global thread-safe cache for resolving fonts
/// from file path
#[derive(Clone)]
pub struct FontCache {
    pub cache: Arc<Mutex<HashMap<String, Font<'static, 'static>>>>,
}

/// Returns the font cache as a singleton
pub fn font_cache() -> FontCache {
    static mut SINGLETON: *const FontCache = 0 as *const FontCache;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
                           let singleton =
                               FontCache { cache: Arc::new(Mutex::new(HashMap::new())) };

                           SINGLETON = mem::transmute(Box::new(singleton));

                           // TODO(DarinM223): clean up memory after exit
                       });

        (*SINGLETON).clone()
    }
}

/// A global thread-safe cache for resolving sprites
/// from file path
#[derive(Clone)]
pub struct SpriteCache {
    pub cache: Arc<Mutex<HashMap<String, Sprite>>>,
}

/// Returns the sprite cache as a singleton
pub fn sprite_cache() -> SpriteCache {
    static mut SINGLETON: *const SpriteCache = 0 as *const SpriteCache;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
                           let singleton =
                               SpriteCache { cache: Arc::new(Mutex::new(HashMap::new())) };

                           SINGLETON = mem::transmute(Box::new(singleton));

                           // TODO(DarinM223): clean up memory after exit
                       });

        (*SINGLETON).clone()
    }
}
