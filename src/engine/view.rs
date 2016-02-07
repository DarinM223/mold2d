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
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction>;
}

pub trait Actor {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64);

    /// Called every frame to update an actor
    fn update(&mut self,
              context: &mut Context,
              other_actors: Vec<&mut Box<Actor>>,
              elapsed: f64)
              -> Option<ActorAction>;

    /// Sets the position of the actor
    fn set_position(&mut self, position: (i32, i32));

    /// Gets the position of the actor
    fn position(&self) -> (i32, i32);

    /// Returns the bounding box for the actor for collision detection
    fn bounding_box(&self) -> Rect;
}
