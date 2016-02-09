use actors::asteroid::Asteroid;
use actors::block::Block;
use engine::actor_manager::ActorManager;
use engine::context::Context;
use engine::level;
use engine::quadtree::Quadtree;
use engine::view::{Actor, ActorAction, ActorType, View, ViewAction};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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
        let mut actors = level::load_level(path,
                                           actor_for_token,
                                           &mut viewport,
                                           &mut context.renderer,
                                           60.0)
                             .unwrap();
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
        context.renderer.set_draw_color(Color::RGB(135, 206, 250));
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

        {
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
                let actor = self.actors.get_mut(key);
                if let Some(actor) = actor {
                    // Only check collisions for players and enemies
                    if actor.data().actor_type == ActorType::Player ||
                       actor.data().actor_type == ActorType::Enemy {
                        let collided_actors = quadtree.retrieve(&actor.data().rect)
                                                      .into_iter()
                                                      .map(|act| act.clone())
                                                      .collect::<Vec<_>>();
                        actions.extend(actor.update(context, &collided_actors, elapsed));
                    }
                }
            }
        }

        // apply actor actions to view
        while !actions.is_empty() {
            let action = actions.pop();
            match action {
                Some(ActorAction::AddActor(actor)) => self.actors.add(actor),
                Some(ActorAction::SetViewport(x, y)) => {
                    if y <= (context.window.height as i32) - ((context.window.height / 2) as i32) {
                        self.viewport.set_position((x, y));
                    }
                }
                _ => {}
            }
        }

        None
    }
}
