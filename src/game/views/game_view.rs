use actions::{ActorMessage, ActorType};
use actors::block::{GroundBlockMid, GroundBlockTop, StartBlock, StoneBlock};
use actors::coin::Coin;
use actors::koopa::Koopa;
use actors::player::Player;
use engine::actor_manager::{ActorFromToken, ActorManager};
use engine::collision::Collision;
use engine::context::Context;
use engine::font;
use engine::level;
use engine::quadtree::Quadtree;
use engine::view::{Actor, View, ViewAction};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use views::background_view::BackgroundView;

pub struct GameActorGenerator;
impl ActorFromToken<ActorType, ActorMessage> for GameActorGenerator {
    fn actor_from_token(&self,
                        token: char,
                        id: i32,
                        position: (i32, i32),
                        renderer: &mut Renderer)
                        -> Box<Actor<ActorType, ActorMessage>> {
        match token {
            'P' => Box::new(Player::new(id, position, renderer, 30.)),
            'C' => Box::new(Coin::new(id, position, renderer, 20.)),
            'K' => Box::new(Koopa::new(id, position, renderer, 30.)),
            'S' => Box::new(StartBlock::new(id, position, renderer, 1.)),
            '=' => Box::new(GroundBlockTop::new(id, position, renderer, 1.)),
            '-' => Box::new(GroundBlockMid::new(id, position, renderer, 1.)),
            '_' => Box::new(StoneBlock::new(id, position, renderer, 1.)),
            _ => panic!("Actor not implemented for token!"),
        }
    }
}

fn handle_message(actors: &mut ActorManager<ActorType, ActorMessage>,
                  viewport: &mut Viewport,
                  c: &mut Context,
                  action: &ActorMessage) {
    use actions::ActorMessage::*;

    match *action {
        AddActor(token, pos) => actors.add(token, pos, &mut c.renderer),
        RemoveActor(id) => actors.remove(id),
        SetViewport(x, y) => viewport.set_position((x, y)),
        DamageActor(id, damage) => {
            let message = actors.get_mut(id).unwrap().handle_message(&DamageActor(id, damage));
            handle_message(actors, viewport, c, &message);
        }
        // TODO(DarinM223): change this to check # of lives left and if
        // it is 0, display the game over screen, otherwise display the level screen again
        PlayerDied => println!("Oh no! The player died!"),
        _ => {}
    }
}

/// The main game view used for
/// the actual gameplay
pub struct GameView {
    actors: ActorManager<ActorType, ActorMessage>,
    viewport: Viewport,
    level_path: String,
}

impl GameView {
    pub fn new(path: &str, context: &mut Context) -> GameView {
        let actor_generator = Box::new(GameActorGenerator);
        let level_result = level::load_level(path,
                                             actor_generator,
                                             &mut context.renderer,
                                             &context.window);
        let (actors, viewport) = level_result.unwrap();

        if context.score.score("GAME_SCORE") == None {
            context.score.add_score("GAME_SCORE");
        }

        GameView {
            actors: actors,
            viewport: viewport,
            level_path: path.to_owned(),
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
            let font_sprite = font::text_sprite(&context.renderer,
                                                &score_text[..],
                                                "assets/belligerent.ttf",
                                                32,
                                                Color::RGB(0, 255, 0))
                                  .unwrap();
            font::render_text(&mut context.renderer, font_sprite, (100, 100));
        }
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> Option<ViewAction> {
        if context.events.event_called("QUIT") || context.events.event_called("ESC") {
            return Some(ViewAction::Quit);
        }

        if context.events.event_called_once("ENTER") {
            return Some(ViewAction::ChangeView(Box::new(BackgroundView)));
        }

        {
            // TODO(DarinM223): eventually avoid creating the quadtree every frame
            let window_rect = Rect::new_unwrap(0, 0, context.window.width, context.window.height);
            let viewport_clone = self.viewport.clone();
            let mut quadtree = Quadtree::new(window_rect, &viewport_clone);
            let mut keys = Vec::new();

            for (key, actor) in &mut self.actors.actors {
                let data = actor.data().clone();

                if let Some(_) = self.viewport.constrain_to_viewport(&data.rect) {
                    keys.push(key.clone());
                    quadtree.insert(data);
                }
            }

            for key in keys {
                let actor = self.actors.temp_remove(key);

                if let Some(mut actor) = actor {
                    if actor.data().checks_collision == true {
                        // only check collisions for certain actors
                        let collided_actors = quadtree.retrieve(&actor.data().rect)
                                                      .into_iter()
                                                      .map(|act| act.clone())
                                                      .collect::<Vec<_>>();
                        for other_actor in collided_actors {
                            if let Some(direction) = actor.collides_with(&other_actor) {
                                let message = actor.on_collision(context, other_actor, direction);
                                handle_message(&mut self.actors,
                                               &mut self.viewport,
                                               context,
                                               &message);
                            }
                        }
                    }

                    // update the actor
                    let message = actor.update(context, elapsed);
                    handle_message(&mut self.actors, &mut self.viewport, context, &message);
                    self.actors.add_existing(actor.data().id, actor);
                }
            }
        }

        None
    }
}
