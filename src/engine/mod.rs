//! The core 2D game engine built from scratch using SDL for graphics and windowing
//!
//! What the engine does (or should do):
//! Abstracts the event loop
//! Reads keyboard mappings from files
//! Handles keyboard inputs based on the mapping
//! Handles sprite and view rendering
//! Includes a renderer interface to render sprites and backgrounds
//! Uses a grid based map system with scrolling support
//! Loads maps from text files
//! A point system
//! A main menu rendering system

#[macro_use]
pub mod sprite;

pub mod context;
pub mod event_loop;
pub mod events;
pub mod frame_timer;
pub mod geo_utils;
pub mod keyboard_mappings;

pub mod view;
