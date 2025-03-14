use crate::events::Events;
use crate::score::Score;
use sdl2::render::Canvas;

/// Represents a SDL window to render
pub struct Window {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

/// Contains the main context variables for the game
/// like the canvas and the events triggered
pub struct Context {
    pub events: Events,
    pub canvas: Canvas<sdl2::video::Window>,
    pub window: Window,
    pub score: Score,
}

impl Context {
    /// Creates a new context given the path for the keyboard configuration
    /// and the sdl canvas
    pub fn new(window: Window, events: Events, canvas: Canvas<sdl2::video::Window>) -> Context {
        Context {
            window,
            events,
            canvas,
            score: Score::new(),
        }
    }
}
