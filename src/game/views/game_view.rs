use actors::asteroid::Asteroid;
use actors::block::Block;
use engine::context::Context;
use engine::level;
use engine::view::{Actor, ActorAction, View, ViewAction};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use std::collections::VecDeque;
use views::background_view::BackgroundView;

level_token_config! {
    '+' => Asteroid,
    'P' => Asteroid,
    '=' => Block
}

pub enum GameState {
    Normal,
    Paused,
    Slowed(f64),
}

/// The main game view used for 
/// the actual gameplay
pub struct GameView {
    state: GameState,
    actors: VecDeque<Box<Actor>>,
    viewport: Viewport,
}

impl GameView {
    pub fn new(path: &str, context: &mut Context) -> GameView {
        let mut viewport = Viewport::new(&context.window, (0, 0));
        let actors: VecDeque<Box<Actor>> = level::load_level(path,
                                                             actor_for_token,
                                                             &mut viewport,
                                                             &mut context.renderer,
                                                             60.0)
                                               .into_iter()
                                               .flat_map(|actor| actor)
                                               .collect();
        GameView {
            state: GameState::Normal,
            actors: actors,
            viewport: viewport,
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
            actor.render(context, &mut self.viewport, elapsed);
        }
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction> {
        if context.events.event_called("QUIT") || context.events.event_called("ESC") {
            return Some(ViewAction::Quit);
        }

        if context.events.event_called_once("ENTER") {
            return Some(ViewAction::ChangeView(Box::new(BackgroundView)));
        }

        let mut actions = Vec::new();

        // update contained actors
        for _ in 0..self.actors.len() {
            if let Some(mut actor) = self.actors.pop_front() {
                let action = actor.update(context,
                                          self.actors.iter_mut().collect::<Vec<_>>(),
                                          elapsed);
                if let Some(action) = action {
                    actions.push(action);
                }

                self.actors.push_back(actor);
            }
        }

        // apply actor actions to view
        while !actions.is_empty() {
            let action = actions.pop();
            match action {
                Some(ActorAction::AddActor(actor)) => self.actors.push_front(actor),
                _ => {}
            }
        }

        None
    }
}
