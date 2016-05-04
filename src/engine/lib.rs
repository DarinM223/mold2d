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
//!
//! Notes: The coordinate system is so that up is a negative change in the
//! y axis, down is a positive change in the y axis, left is a negative
//! change in the x axis, and right is a positive change in the x axis.

#![feature(custom_attribute)]
#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

extern crate libc;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

pub mod actor_manager;
pub mod cache;
pub mod collision;
pub mod context;
pub mod event_loop;
pub mod events;
pub mod font;
pub mod level;
pub mod quadtree;
pub mod raycast;
pub mod score;
pub mod sprite;
pub mod vector;
pub mod view;
pub mod viewport;

pub use actor_manager::{ActorFromToken, ActorManager};
pub use collision::{BoundingBox, Collision, CollisionSide};
pub use context::{Context, Window};
pub use events::Events;
pub use quadtree::Quadtree;
pub use raycast::{Polygon, Segment};
pub use score::Score;
pub use sprite::{AnimatedSprite, Animation, AnimationData, AnimationManager, Direction,
                 Renderable, Sprite, SpriteRectangle};
pub use vector::Vector2D;
pub use view::{Actor, ActorData, PositionChange, View, ViewAction};
pub use viewport::Viewport;
