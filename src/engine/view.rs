use collision::{BoundingBox, CollisionSide};
use context::Context;
use sdl2::rect::Rect;
use viewport::Viewport;

/// Actions that the view would want the event loop to do
pub enum ViewAction {
    Quit,
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
    pub id: i32,
    pub state: u32,
    pub damage: i32,
    pub collision_filter: u8,
    pub rect: Rect,
    pub bounding_box: Option<BoundingBox>,
    pub actor_type: Type,
}

/// Represents a change of position
pub struct PositionChange {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl PositionChange {
    pub fn new() -> PositionChange {
        PositionChange {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }

    pub fn left(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x + amount,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn right(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x - amount,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn up(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y - amount,
            w: self.w,
            h: self.h,
        }
    }

    pub fn down(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y + amount,
            w: self.w,
            h: self.h,
        }
    }
}

pub trait Actor<Type, Message> {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Handle a message sent by another actor
    fn handle_message(&mut self, message: &Message) -> Message;

    /// Returns the side of the collision if actor collides with another actor
    fn collides_with(&mut self, other_actor: &ActorData<Type>) -> Option<CollisionSide>;

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Message;

    /// Gets the actor data
    fn data(&mut self) -> ActorData<Type>;

    /// Change position
    fn change_pos(&mut self, change: &PositionChange);
}
