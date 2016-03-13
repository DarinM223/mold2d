use collision::{BoundingBox, CollisionSide};
use context::Context;
use sdl2::rect::Rect;
use viewport::Viewport;

/// Actions that the view would want the event loop to do
pub enum ViewAction {
    Quit,
    ChangeView(Box<View>),
}

/// Actions that the actor would want the parent view to do
pub enum ActorAction {
    AddActor(char, (i32, i32)),
    RemoveActor(i32),
    SetViewport(i32, i32),
    MultipleActions(Vec<Box<ActorAction>>),
    PlayerDied,
    None,
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction>;
}

#[derive(Clone, PartialEq)]
pub enum ActorType {
    Item,
    Block,
    Player,
    Enemy,
}

/// The data contained in an actor
#[derive(Clone, PartialEq)]
pub struct ActorData {
    pub id: i32,
    pub state: u32,
    pub damage: i32,
    pub checks_collision: bool,
    pub rect: Rect,
    pub bounding_box: Option<BoundingBox>,
    pub actor_type: ActorType,
}

pub trait Actor {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Called when an actor collides with another actor
    fn on_collision(&mut self,
                    context: &mut Context,
                    other_actor: ActorData,
                    side: CollisionSide)
                    -> ActorAction;

    /// Returns the side of the collision if actor collides with another actor
    fn collides_with(&mut self, other_actor: &ActorData) -> Option<CollisionSide>;

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorAction;

    /// Gets the actor data
    fn data(&mut self) -> ActorData;
}
