use actors::block::{GroundBlockMid, GroundBlockTop, StartBlock, StoneBlock};
use actors::coin::Coin;
use actors::koopa::Koopa;
use actors::player::Player;
use engine::{Actor, ActorData, ActorFromToken, ActorManager, CollisionSide, Viewport, Context,
             PositionChange};
use sdl2::render::Renderer;

/// Actions for an actor to process
#[derive(Clone, Debug, PartialEq)]
pub enum ActorAction {
    DamageActor(i32),
    Collision(ActorType, u8),
}

/// Actor messages
#[derive(Clone, Debug, PartialEq)]
pub enum ActorMessage {
    AddActor(char, (i32, i32)),
    RemoveActor(i32),
    SetViewport(i32, i32),
    ActorAction(i32, i32, ActorAction),
    MultipleMessages(Vec<Box<ActorMessage>>),
    PlayerDied,
    UpdateScore(i32),
    None,
}

/// Actor types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActorType {
    Item,
    Block,
    Player,
    Enemy,
}

// Handlers

#[inline]
pub fn actor_from_token(token: char,
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

#[inline]
pub fn handle_collision(actor: &mut Box<Actor<ActorType, ActorMessage>>,
                        other: &ActorData<ActorType>,
                        direction: CollisionSide,
                        actors: &mut ActorManager<ActorType, ActorMessage>,
                        viewport: &mut Viewport,
                        context: &mut Context) {
    let data = actor.data();
    // TODO(DarinM223): remove hack that fixes bug with block collision
    // detection
    if data.actor_type != ActorType::Block {
        while actor.collides_with(other) == Some(direction) {
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

    let collision = ActorAction::Collision(other.actor_type, direction);
    let message = ActorMessage::ActorAction(other.id, data.id, collision);
    let response = actor.handle_message(&message);

    let other_coll = ActorAction::Collision(data.actor_type, rev_dir);
    let other_msg = ActorMessage::ActorAction(data.id, other.id, other_coll);

    handle_message(actor, actors, viewport, context, &response);
    handle_message(actor, actors, viewport, context, &other_msg);
}

#[inline]
fn handle_message(curr_actor: &mut Box<Actor<ActorType, ActorMessage>>,
                  actors: &mut ActorManager<ActorType, ActorMessage>,
                  viewport: &mut Viewport,
                  context: &mut Context,
                  action: &ActorMessage) {
    use actions::ActorMessage::*;

    match *action {
        AddActor(token, pos) => actors.add(token, pos, &mut context.renderer),
        RemoveActor(id) => actors.remove(id),
        UpdateScore(amount) => context.score.increment_score("GAME_SCORE", amount),
        MultipleMessages(ref messages) => {
            for message in messages {
                handle_message(curr_actor, actors, viewport, context, message);
            }
        }
        ref action @ ActorAction(..) => {
            if let ActorAction(_, id, _) = *action {
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
