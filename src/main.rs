#![feature(convert, custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

mod engine;
mod game;

fn main() {
    let window = engine::event_loop::Window {
        title: "Window".to_owned(),
        width: 800,
        height: 600,
    };
    engine::event_loop::create_event_loop(window);
}
