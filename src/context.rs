use crate::events::Events;
use crate::score::Score;
use sdl2::render::Renderer;

/// Represents a SDL window to render
pub struct Window {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
}

/// Contains the main context variables for the game
/// like the renderer and the events triggered
pub struct Context<'a> {
    pub events: Events,
    pub renderer: Renderer<'a>,
    pub window: Window,
    pub score: Score,
}

impl<'a> Context<'a> {
    /// Creates a new context given the path for the keyboard configuration
    /// and the sdl renderer
    pub fn new(window: Window, events: Events, renderer: Renderer<'a>) -> Context<'a> {
        Context {
            window,
            events,
            renderer,
            score: Score::new(),
        }
    }
}
