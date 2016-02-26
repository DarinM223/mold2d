use actors::block::{GroundBlockMid, GroundBlockTop, StartBlock, StoneBlock};
use actors::coin::Coin;
use actors::player::Player;
use engine::actor_manager::{ActorFromToken, ActorManager};
use engine::collision::Collision;
use engine::context::Context;
use engine::level;
use engine::quadtree::Quadtree;
use engine::view::{Actor, ActorAction, View, ViewAction};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use views::background_view::BackgroundView;

pub struct GameActorGenerator;
impl ActorFromToken for GameActorGenerator {
    fn actor_from_token(&self,
                        token: char,
                        id: i32,
                        position: (i32, i32),
                        renderer: &mut Renderer,
                        fps: f64)
                        -> Box<Actor> {
        match token {
            'P' => Box::new(Player::new(id, position, renderer, fps)),
            'C' => Box::new(Coin::new(id, position, renderer, fps)),
            'S' => Box::new(StartBlock::new(id, position, renderer, fps)),
            '=' => Box::new(GroundBlockTop::new(id, position, renderer, fps)),
            '-' => Box::new(GroundBlockMid::new(id, position, renderer, fps)),
            '_' => Box::new(StoneBlock::new(id, position, renderer, fps)),
            _ => panic!("Actor not implemented for token!"),
        }
    }
}

/// The main game view used for
/// the actual gameplay
pub struct GameView {
    actors: ActorManager,
    viewport: Viewport,
}

impl GameView {
    pub fn new(path: &str, context: &mut Context) -> GameView {
        let actor_generator = Box::new(GameActorGenerator);
        let level_result = level::load_level(path,
                                             actor_generator,
                                             &mut context.renderer,
                                             &context.window,
                                             60.0);
        let (actors, viewport) = level_result.unwrap();

        if context.score.score("GAME_SCORE") == None {
            context.score.add_score("GAME_SCORE");
        }

        GameView {
            actors: actors,
            viewport: viewport,
        }
    }

    pub fn apply_action(&mut self, c: &mut Context, action: &ActorAction) {
        use engine::view::ActorAction::*;

        match *action {
            AddActor(token, pos) => self.actors.add(token, pos, &mut c.renderer, 60.),
            RemoveActor(id) => self.actors.remove(id),
            SetViewport(x, y) => self.viewport.set_position((x, y)),
            _ => {}
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

        if let Some(score) = context.score.score("GAME_SCORE") {
            let score_text = format!("Score: {}", score);
            let font_sprite = context.font_renderer
                                     .text_sprite(&context.renderer,
                                                  &score_text[..],
                                                  "assets/belligerent.ttf",
                                                  32,
                                                  Color::RGB(0, 255, 0))
                                     .unwrap();
            context.font_renderer.render_text(&mut context.renderer, font_sprite, (100, 100));
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
                            if let Some(direction) = actor.collides_with(other_actor.clone()) {
                                actions.push(actor.on_collision(context, other_actor, direction));
                            }
                        }
                    }

                    // update the actor
                    actions.push(actor.update(context, elapsed));
                }
            }
        }

        // apply actions sent by the actors
        for action in &actions {
            self.apply_action(context, action);
        }

        None
    }
}
