#![feature(custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

mod engine;
mod game;

fn main() {
    println!("Hello world!");
}
