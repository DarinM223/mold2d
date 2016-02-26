//! A demo game to demonstrate mold2d

#![feature(custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use(block)]
extern crate engine;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

pub mod actors;
pub mod views;
