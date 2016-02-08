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
    AddActor(Box<Actor>),
    SetViewport(i32, i32),
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction>;
}

#[derive(Clone, PartialEq)]
pub enum ActorType {
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
    pub rect: Rect,
    pub actor_type: ActorType,
}

pub trait Actor {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Called every frame to update an actor
    fn update(&mut self,
              context: &mut Context,
              other_actors: &Vec<ActorData>,
              elapsed: f64)
              -> Vec<ActorAction>;

    /// Gets the actor data
    fn data(&self) -> ActorData;

    fn set_position(&mut self, position: (i32, i32));
}
