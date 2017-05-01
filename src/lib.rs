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

#![feature(drop_types_in_const)]

extern crate libc;
extern crate sdl2;

#[macro_use]
pub mod block;

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
pub mod viewport;

pub use actor_manager::{ActorFromToken, ActorManager};
pub use collision::{BoundingBox, Collision, CollisionSide};
pub use context::{Context, Window};
pub use events::Events;
pub use quadtree::Quadtree;
pub use raycast::{Polygon, Segment};
pub use score::Score;
pub use sprite::{AnimatedSprite, Animations, Direction, Renderable, Sprite, Spritesheet,
                 SpritesheetConfig, SpriteRectangle};
pub use sdl2::ttf::FontError;
pub use vector::{PositionChange, Vector2D};
pub use viewport::Viewport;

use sdl2::rect::Rect;

/// Handler for a view to deal with actor messages
pub type MessageHandler<A: Actor + ?Sized> = Box<Fn(&mut Box<A>,
                                                    &mut ActorManager<A>,
                                                    &mut Viewport,
                                                    &mut Context,
                                                    &A::Message)>;

/// Actions that the view would want the event loop to do
pub enum ViewAction {
    /// Quit the game
    Quit,
    /// Switch to a different view
    ChangeView(Box<View>),
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction>;
}

/// The data contained in an actor
#[derive(Clone, PartialEq)]
pub struct ActorData<Type> {
    /// The id of the actor given by the actor manager
    pub id: i32,
    /// The current state of the actor as a number
    pub state: u32,
    /// The damage that the actor has taken so far
    pub damage: i32,
    /// A byte that contains the sides that
    /// other actors can collide into
    pub collision_filter: u8,
    /// If true, on collision the actor would be
    /// moved away from the collision
    pub resolves_collisions: bool,
    /// The sprite rectangle
    pub rect: Rect,
    /// The current bounding box for the actor
    pub bounding_box: Option<BoundingBox>,
    /// The type of the actor
    pub actor_type: Type,
}

/// A game object that supports sending and receiving messages
pub trait Actor {
    type Type;
    type Message;

    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Handle a message sent by another actor
    fn handle_message(&mut self, message: &Self::Message) -> Self::Message;

    /// Returns the side of the collision if actor collides with another actor
    fn collides_with(&mut self, other_actor: &ActorData<Self::Type>) -> Option<CollisionSide>;

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> PositionChange;

    /// Gets the actor data
    fn data(&mut self) -> ActorData<Self::Type>;
}
