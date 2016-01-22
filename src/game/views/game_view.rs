use engine::context::Context;
use engine::view::{Actor, View, ViewAction};
use sdl2::pixels::Color;

pub enum GameState {
    Normal,
    Paused,
    Slowed(f64),
}

/// A standard game view with sprites
/// meant to be plugged into a custom view
pub struct GameView {
    state: GameState,
    actors: Vec<Box<Actor>>,
}

impl GameView {
    pub fn new() -> GameView {
        GameView {
            state: GameState::Normal,
            actors: Vec::new(),
        }
    }
}

impl View for GameView {
    fn render(&mut self, context: &mut Context, elapsed: f64) {
        // start off with a black screen
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        // render contained actors
        for actor in &mut self.actors {
            actor.render(context, elapsed);
        }
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction {
        if context.events.event_called("QUIT") || context.events.event_called("ESC") {
            return ViewAction::Quit;
        }

        if context.events.event_called("ENTER") {
            println!("The enter key is pressed!");
        }

        if context.events.event_called_once("ENTER") {
            println!("Enter was pressed once!");
            // TODO: add a random asteroid
        }

        // update contained actors
        for actor in &mut self.actors {
            actor.update(context, elapsed);
        }

        ViewAction::None
    }
}
