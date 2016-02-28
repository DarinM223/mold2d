use libc;
use sdl2_ttf::Font;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex, ONCE_INIT, Once};

#[derive(Clone)]
pub struct FontCache {
    pub cache: Arc<Mutex<HashMap<String, Font>>>,
}

pub fn font_cache() -> FontCache {
    static mut SINGLETON: *const FontCache = 0 as *const FontCache;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let singleton = FontCache { cache: Arc::new(Mutex::new(HashMap::new())) };

            SINGLETON = mem::transmute(Box::new(singleton));
        });

        (*SINGLETON).clone()
    }
}
