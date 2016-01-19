use engine::context::Context;

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    /// Called every frame to render a view
    fn render(&mut self, context: &mut Context, elapsed: f64);

    /// Called every frame to update a view
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction;
}

pub enum GameState {
    Normal,
    Paused,
    Slowed(f64),
}

/// A standard game view with sprites
/// meant to be plugged into a custom view
pub struct GameView {
    state: GameState,
}

impl GameView {
    pub fn new() -> GameView {
        GameView { state: GameState::Normal }
    }
}

impl View for GameView {
    fn render(&mut self, context: &mut Context, elapsed: f64) {
        // TODO: implement this
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction {
        // TODO: implement this
        // TODO: handle keyboard events
        // TODO: dispatch events to sprites
        ViewAction::None
    }
}
