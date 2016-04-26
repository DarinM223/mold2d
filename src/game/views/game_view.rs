use actions::{ActorAction, ActorMessage, ActorType, GameActorGenerator};
use engine::collision::print_collision_side_u8;
use engine::font;
use engine::level;
use engine::{Actor, ActorManager, Collision, CollisionSide, Context, PositionChange, Quadtree,
             View, ViewAction, Viewport};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use views::background_view::BackgroundView;

fn handle_message(curr_actor: &mut Box<Actor<ActorType, ActorMessage>>,
                  actors: &mut ActorManager<ActorType, ActorMessage>,
                  viewport: &mut Viewport,
                  context: &mut Context,
                  action: &ActorMessage) {
    use actions::ActorMessage::*;

    match *action {
        AddActor(token, pos) => actors.add(token, pos, &mut context.renderer),
        RemoveActor(id) => actors.remove(id),
        SetViewport(x, y) => viewport.set_position((x, y)),
        UpdateScore(amount) => context.score.increment_score("GAME_SCORE", amount),
        MultipleMessages(ref messages) => {
            for message in messages {
                handle_message(curr_actor, actors, viewport, context, message);
            }
        }
        ref action @ ActorAction(_, _) => {
            if let ActorAction(id, _) = *action {
                let message = if curr_actor.data().id == id {
                    curr_actor.handle_message(&action)
                } else if let Some(ref mut actor) = actors.get_mut(id) {
                    actor.handle_message(&action)
                } else {
                    ActorMessage::None
                };
                handle_message(curr_actor, actors, viewport, context, &message);
            }
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

        // render score
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
            let mut keys = Vec::with_capacity(self.actors.actors.len());

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
                    let data = actor.data();
                    if data.collision_filter != 0 {
                        // only check collisions for certain actors
                        let collided_actors = quadtree.retrieve(&data.rect)
                                                      .into_iter()
                                                      .map(|act| act.clone())
                                                      .collect::<Vec<_>>();
                        for other in collided_actors {
                            if let Some(direction) = actor.collides_with(&other) {
                                // TODO(DarinM223): remove hack that fixes bug with block collision
                                // detection
                                if data.actor_type != ActorType::Block {
                                    while actor.collides_with(&other) == Some(direction) {
                                        match direction {
                                            CollisionSide::Top => {
                                                actor.change_pos(&PositionChange::new().down(1));
                                            }
                                            CollisionSide::Bottom => {
                                                actor.change_pos(&PositionChange::new().up(1));
                                            }
                                            CollisionSide::Left => {
                                                actor.change_pos(&PositionChange::new().right(1));
                                            }
                                            CollisionSide::Right => {
                                                actor.change_pos(&PositionChange::new().left(1));
                                            }
                                        }
                                    }

                                    if direction == CollisionSide::Bottom {
                                        actor.change_pos(&PositionChange::new().down(1));
                                    }
                                }

                                let direction = direction & other.collision_filter;
                                let rev_dir = CollisionSide::reverse_u8(direction);

                                if data.actor_type == ActorType::Player {
                                    print!("(P): ");
                                    print_collision_side_u8(direction);
                                    println!("");
                                    print!("(O): ");
                                    print_collision_side_u8(rev_dir);
                                    println!("");
                                }

                                let collision = ActorAction::Collision(other.actor_type, direction);
                                let message = ActorMessage::ActorAction(other.id, collision);
                                let response = actor.handle_message(&message);

                                let other_coll = ActorAction::Collision(data.actor_type, direction);
                                let other_msg = ActorMessage::ActorAction(data.id, other_coll);

                                handle_message(&mut actor,
                                               &mut self.actors,
                                               &mut self.viewport,
                                               context,
                                               &response);
                                handle_message(&mut actor,
                                               &mut self.actors,
                                               &mut self.viewport,
                                               context,
                                               &other_msg);
                            }
                        }
                    }

                    // update the actor
                    let message = actor.update(context, elapsed);
                    handle_message(&mut actor,
                                   &mut self.actors,
                                   &mut self.viewport,
                                   context,
                                   &message);
                    self.actors.temp_reinsert(actor.data().id, actor);
                }
            }
        }

        None
    }
}
