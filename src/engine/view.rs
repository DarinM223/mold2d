use collision::BoundingBox;
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

pub trait Actor<Type, Message> {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Handle a message sent by another actor
    fn handle_message(&mut self, message: &Message) -> Message;

    /// Called when an actor collides with another actor
    fn on_collision(&mut self,
                    context: &mut Context,
                    other_actor: ActorData<Type>,
                    side: u8)
                    -> Message;

    /// Returns the side of the collision if actor collides with another actor
    fn collides_with(&mut self, other_actor: &ActorData<Type>) -> Option<u8>;

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Message;

    /// Gets the actor data
    fn data(&mut self) -> ActorData<Type>;
}
