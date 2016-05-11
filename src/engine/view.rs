use actor_manager::ActorManager;
use collision::{BoundingBox, CollisionSide};
use context::Context;
use sdl2::rect::Rect;
use vector::PositionChange;
use viewport::Viewport;

/// Handler for a view to deal with actor messages
pub type MessageHandler<Type, Message> = Box<Fn(&mut Box<Actor<Type, Message>>,
                                                &mut ActorManager<Type, Message>,
                                                &mut Viewport,
                                                &mut Context,
                                                &Message)>;

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

pub trait Actor<Type, Message> {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Handle a message sent by another actor
    fn handle_message(&mut self, message: &Message) -> Message;

    /// Returns the side of the collision if actor collides with another actor
    fn collides_with(&mut self, other_actor: &ActorData<Type>) -> Option<CollisionSide>;

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> PositionChange;

    /// Gets the actor data
    fn data(&mut self) -> ActorData<Type>;

    /// Change position
    fn change_pos(&mut self, change: &PositionChange);
}
