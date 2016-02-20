use actors::asteroid::Asteroid;
use actors::block::Block;
use actors::coin::Coin;
use engine::actor_manager::ActorManager;
use engine::collision::Collision;
use engine::context::Context;
use engine::level;
use engine::quadtree::Quadtree;
use engine::view::{Actor, ActorAction, View, ViewAction};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use views::background_view::BackgroundView;

level_token_config! {
    '+' => Asteroid,
    'P' => Asteroid,
    'C' => Coin,
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
        let level_result = level::load_level(path,
                                             actor_for_token,
                                             &mut context.renderer,
                                             &context.window,
                                             60.0);
        let (actors, viewport) = level_result.unwrap();

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
            if let Some(_) = self.viewport.constrain_to_viewport(&actor.data().rect) {
                actor.render(context, &mut self.viewport, elapsed);
            }
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
            // TODO(DarinM223): eventually avoid creating the quadtree every frame
            let window_rect = Rect::new_unwrap(0, 0, context.window.width, context.window.height);
            let mut quadtree = Quadtree::new(window_rect, &self.viewport);
            let mut keys = Vec::new();

            for (key, actor) in &self.actors.actors {
                keys.push(key.clone());

                let data = actor.data().clone();

                if let Some(_) = self.viewport.constrain_to_viewport(&data.rect) {
                    quadtree.insert(data);
                }
            }

            for key in keys {
                let actor = self.actors.get_mut(key);

                if let Some(actor) = actor {
                    // only check collisions for certain actors
                    if actor.data().checks_collision == true {
                        let collided_actors = quadtree.retrieve(&actor.data().rect)
                                                      .into_iter()
                                                      .map(|act| act.clone())
                                                      .collect::<Vec<_>>();
                        for other_actor in collided_actors {
                            let rect = actor.data().rect;
                            if let Some(direction) = rect.collides_with(other_actor.rect) {
                                actor.on_collision(other_actor, direction);
                            }
                        }
                    }

                    // update the actor
                    actions.extend(actor.update(context, elapsed));
                }
            }
        }

        // apply actor actions to view
        while !actions.is_empty() {
            let action = actions.pop();
            match action {
                Some(ActorAction::AddActor(actor)) => self.actors.add(actor),
                Some(ActorAction::SetViewport(x, y)) => self.viewport.set_position((x, y)),
                _ => {}
            }
        }

        None
    }
}
