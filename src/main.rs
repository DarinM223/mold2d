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
    println!("Hello world!");
}
