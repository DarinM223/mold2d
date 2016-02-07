//! The core 2D game engine built from scratch using SDL for graphics and windowing
//!
//! What the engine does (or should do):
//! Abstracts the event loop
//! Reads keyboard mappings from files
//! Handles keyboard inputs based on the mapping
//! Handles sprite and view rendering
//! Includes a renderer interface to render sprites and backgrounds
//! Uses a grid based map system with scrolling support
//! Loads level maps from text files
//! A point system
//! A main menu rendering system

#![feature(custom_attribute, plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

#[macro_use]
pub mod level;
#[macro_use]
pub mod sprite;

pub mod context;
pub mod event_loop;
pub mod events;
pub mod physics;
pub mod view;
pub mod viewport;
