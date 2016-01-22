use engine::context::Context;
use engine::view::{Actor, View, ViewAction};
use game::actors::asteroid::Asteroid;
use rand::random;
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

        if context.events.event_called_once("ENTER") {
            let mut new_asteroid = Asteroid::new(&mut context.renderer, 60 as f64);
            let max_width = context.window.width - 100;
            let max_height = context.window.height - 100;

            new_asteroid.rect.x = (random::<u32>() % max_width) as i32 + 1;
            new_asteroid.rect.y = (random::<u32>() % max_height) as i32 + 1;

            self.actors.push(Box::new(new_asteroid));
        }

        let mut actions = Vec::new();

        // update contained actors
        for actor in &mut self.actors {
            actions.push(actor.update(context, elapsed));
        }

        for action in actions {
            match action {
                ViewAction::AddActor(actor) => self.actors.push(actor),
                ViewAction::Quit => return ViewAction::Quit,
                _ => {}
            }
        }

        ViewAction::None
    }
}
