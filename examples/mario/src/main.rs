#![feature(custom_attribute, plugin)]
#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use(block)]
extern crate mold2d;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

pub mod actions;
pub mod actors;
pub mod views;

use mold2d::Window;
use mold2d::event_loop;
use views::game_view::GameView;

fn main() {
    let window = Window {
        title: "Mold2d demo game",
        width: 1024,
        height: 600,
    };

    let result = event_loop::create_event_loop(window, |context| {
        Box::new(GameView::new("levels/level1.txt", context))
    });

    match result {
        Ok(_) => println!("Game exited successfully!"),
        Err(e) => println!("{}", e),
    }
}
