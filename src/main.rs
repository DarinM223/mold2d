#![feature(convert, custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

#[macro_use]
mod engine;

mod game;

// Testing spritesheet! macro
spritesheet! {
    name: Koopa,
    state: KoopaState,
    path: "/assets/foo",
    animations: { 
        Idle: 1..5,
        Walking: 5..10,
        Running: 10..15
    }
}

fn main() {
    // Testing spritesheet! macro
    let koopa = KoopaState::Idle;
    println!("{:?}", koopa);

    // Start the game :)
    game::start();
}
