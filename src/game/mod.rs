//! Trage - A troll rage 2d platformer game
//! An attempt to build a 2d platformer game from scratch
//! using only SDL for graphics

pub mod asteroid;

use engine::event_loop;
use engine::event_loop::Window;

/// Creates the window and starts the game
pub fn start() {
    let window = Window {
        title: "Trage - The troll rage game".to_owned(),
        width: 800,
        height: 600,
    };
    event_loop::create_event_loop(window);
}
