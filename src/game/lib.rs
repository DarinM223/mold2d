//! Trage - A troll rage 2d platformer game
//! An attempt to build a 2d platformer game from scratch
//! using only SDL for graphics

#![feature(custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use(level_token_config, spritesheet)]
extern crate engine;

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

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

    let result = event_loop::create_event_loop(window, |context| {
        Box::new(views::game_view::GameView::new("levels/level1.txt", context))
    });

    match result {
        Ok(_) => println!("Game exited successfully!"),
        Err(e) => println!("{}", e),
    }
}
