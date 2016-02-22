//! Trage - A troll rage 2d platformer game
//! An attempt to build a 2d platformer game from scratch
//! using only SDL for graphics

#![feature(custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]
#[macro_use(level_token_config)]
extern crate engine;

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

pub mod actors;
pub mod views;
