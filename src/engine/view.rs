use engine::context::Context;
use engine::viewport::Viewport;

/// Actions that the view would want the event loop to do
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

/// Actions that the actor would want the parent view to do
pub enum ActorAction {
    None,
    AddActor(Box<Actor>),
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction;
}

pub trait Actor {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, viewport: &Viewport, elapsed: f64);

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorAction;
}
