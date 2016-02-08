use actors::asteroid::Asteroid;
use actors::block::Block;
use engine::actor_manager::ActorManager;
use engine::context::Context;
use engine::level;
use engine::physics::quadtree::Quadtree;
use engine::view::{Actor, ActorAction, ActorData, View, ViewAction};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::VecDeque;
use std::mem;
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
    actors: ActorManager,
    viewport: Viewport,
}

impl GameView {
    pub fn new(path: &str, context: &mut Context) -> GameView {
        let mut viewport = Viewport::new(&context.window, (0, 0));
        let mut actors = ActorManager::new();
        let mut actor_vec: Vec<Box<Actor>> = level::load_level(path,
                                                               actor_for_token,
                                                               &mut viewport,
                                                               &mut context.renderer,
                                                               60.0)
                                                 .unwrap();
        while !actor_vec.is_empty() {
            actors.add(actor_vec.pop().unwrap());
        }

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
        for (_, actor) in &mut self.actors.actors {
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
        let mut quadtree = Quadtree::new(Rect::new_unwrap(0,
                                                          0,
                                                          context.window.width,
                                                          context.window.height),
                                         &self.viewport);
        let mut keys = Vec::new();

        for (key, actor) in &self.actors.actors {
            keys.push(key.clone());

            quadtree.insert(actor.data().clone());
        }

        for key in keys {
            let mut actor = self.actors.get_mut(key);
            if let Some(actor) = actor {
                let mut collided_actors = Vec::new();
                let rects = quadtree.retrieve(&actor.data().rect);
                for rect in rects {
                    collided_actors.push(rect.clone());
                }
                let action = actor.update(context, &collided_actors, elapsed);
                if let Some(action) = action {
                    actions.push(action);
                }
            }
        }

        // apply actor actions to view
        while !actions.is_empty() {
            let action = actions.pop();
            match action {
                Some(ActorAction::AddActor(actor)) => self.actors.add(actor),
                _ => {}
            }
        }

        None
    }
}
