use engine::context::Context;

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64) -> ViewAction;
}
