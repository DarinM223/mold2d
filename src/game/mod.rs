//! Trage - A troll rage 2d platformer game
//! An attempt to build a 2d platformer game from scratch
//! using only SDL for graphics

pub mod actors;
pub mod views;

use engine::context::Window;
use engine::event_loop;

/// Creates the window and starts the game
pub fn start() {
    let window = Window {
        title: "Trage - The troll rage game",
        width: 800,
        height: 600,
    };
    event_loop::create_event_loop(window,
                                  |context| Box::new(views::game_view::GameView::new(context)));
}
