use actors::block::{GroundBlockMid, GroundBlockTop, StartBlock, StoneBlock};
use actors::coin::Coin;
use actors::koopa::Koopa;
use actors::player::Player;
use mold2d;
use mold2d::{ActorManager, CollisionSide, Context, MessageHandler, PositionChange, Viewport};
use sdl2::render::Renderer;

/// Actions for an actor to process
#[derive(Clone, Debug, PartialEq)]
pub enum ActorAction {
    /// A message that applies damage to an actor
    DamageActor(i32),
    /// A message that moves an actor when received
    ChangePosition(PositionChange),
    /// A message sent by an acter when it collides
    /// into another actor
    Collision(ActorType, CollisionSide),
    /// Ask message sent by an actor to ask another
    /// actor if it can bounce on it
    CanBounce,
    /// Response from asked actor to question
    Bounce(bool),
}

/// Actor messages
#[derive(Clone, Debug, PartialEq)]
pub enum ActorMessage {
    AddActor(char, (i32, i32)),
    RemoveActor(i32),
    SetViewport(i32, i32),
    ActorAction {
        send_id: i32,
        recv_id: i32,
        action: ActorAction,
    },
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

pub type Actor = mold2d::Actor<Type = ActorType, Message = ActorMessage>;
pub type ActorData = mold2d::ActorData<ActorType>;

// Handlers

#[inline]
pub fn actor_from_token(
    token: char,
    id: i32,
    position: (i32, i32),
    renderer: &mut Renderer,
) -> Box<Actor> {
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
pub fn handle_message(
    curr_actor_id: i32,
    actors: &mut ActorManager<Actor>,
    viewport: &mut Viewport,
    context: &mut Context,
    action: &ActorMessage,
) {
    use actions::ActorMessage::*;

    match *action {
        AddActor(token, pos) => actors.add(token, pos, &mut context.renderer),
        RemoveActor(id) => actors.remove(id),
        UpdateScore(amount) => context.score.increment_score("GAME_SCORE", amount),
        MultipleMessages(ref messages) => {
            for message in messages {
                handle_message(curr_actor_id, actors, viewport, context, message);
            }
        }
        ActorAction { recv_id, .. } => {
            let message = actors.apply_message(recv_id, action, ActorMessage::None);
            handle_message(curr_actor_id, actors, viewport, context, &message);
        }
        // TODO(DarinM223): change this to check # of lives left and if
        // it is 0, display the game over screen, otherwise display the level screen again
        PlayerDied => println!("Oh no! The player died!"),
        _ => {}
    }
}

/// Moves actor away from collided actor.
#[inline]
pub fn resolve_collision(actor: &mut Box<Actor>, other: &ActorData, direction: CollisionSide) {
    let data = actor.data();
    if data.resolves_collisions {
        while actor.collides_with(other) == Some(direction) {
            let change = match direction {
                CollisionSide::Top => PositionChange::new().down(1),
                CollisionSide::Bottom => PositionChange::new().up(1),
                CollisionSide::Left => PositionChange::new().right(1),
                CollisionSide::Right => PositionChange::new().left(1),
            };

            actor.handle_message(&ActorMessage::ActorAction {
                send_id: -1,
                recv_id: -1,
                action: ActorAction::ChangePosition(change),
            });
        }

        if direction == CollisionSide::Bottom {
            let down_change = PositionChange::new().down(1);
            actor.handle_message(&ActorMessage::ActorAction {
                send_id: -1,
                recv_id: -1,
                action: ActorAction::ChangePosition(down_change),
            });
        }
    }
}

/// Sends collision messages to both of the collided actors.
#[inline]
pub fn handle_collision(
    actor: &ActorData,
    other: &ActorData,
    direction: CollisionSide,
    handler: MessageHandler<Actor>,
    actors: &mut ActorManager<Actor>,
    viewport: &mut Viewport,
    context: &mut Context,
) {
    let direction = direction & other.collision_filter;
    let rev_dir = CollisionSide::reverse_u8(direction);

    if direction != 0 {
        let response = ActorMessage::ActorAction {
            send_id: other.id,
            recv_id: actor.id,
            action: ActorAction::Collision(other.actor_type, CollisionSide::from(direction)),
        };
        let other_msg = ActorMessage::ActorAction {
            send_id: actor.id,
            recv_id: other.id,
            action: ActorAction::Collision(actor.actor_type, CollisionSide::from(rev_dir)),
        };

        (handler)(actor.id, actors, viewport, context, &response);
        (handler)(actor.id, actors, viewport, context, &other_msg);
    }
}
