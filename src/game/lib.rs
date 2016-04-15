//! A demo game to demonstrate mold2d

#![feature(custom_attribute, plugin)]
#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

extern crate engine;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

pub mod actions;
pub mod actors;
pub mod views;
