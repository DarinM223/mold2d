use engine::context::Context;

pub enum ViewAction {
    None,
    Quit,
    AddActor(Box<Actor>),
    ChangeView(Box<View>),
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction;
}

pub trait Actor {
    /// Called every frame to render an actor
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update an actor
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction;
}
