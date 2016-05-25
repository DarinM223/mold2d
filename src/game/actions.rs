use actors::block::{GroundBlockMid, GroundBlockTop, StartBlock, StoneBlock};
use actors::coin::Coin;
use actors::koopa::Koopa;
use actors::player::Player;
use engine::{Actor, ActorManager, CollisionSide, MessageType, PositionChange, Viewport, Context};
use sdl2::render::Renderer;

/// Actions for an actor to process
#[derive(Clone, Debug, PartialEq)]
pub enum ActorAction {
    DamageActor(i32),
    ChangePosition(PositionChange),
    Collision(ActorType, CollisionSide),
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
pub fn handle_message(curr_actor: &mut Box<Actor<ActorType, ActorMessage>>,
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
        ActorAction { recv_id, .. } => {
            let message = if curr_actor.data().id == recv_id {
                curr_actor.handle_message(&action)
            } else if let Some(ref mut actor) = actors.get_mut(recv_id) {
                actor.handle_message(&action)
            } else {
                ActorMessage::None
            };
            handle_message(curr_actor, actors, viewport, context, &message);
        }
        // TODO(DarinM223): change this to check # of lives left and if
        // it is 0, display the game over screen, otherwise display the level screen again
        PlayerDied => println!("Oh no! The player died!"),
        _ => {}
    }
}

#[inline]
pub fn create_msg(message: MessageType<ActorType>) -> ActorMessage {
    match message {
        MessageType::Collision(sender, receiver, direction) => {
            ActorMessage::ActorAction {
                send_id: sender.id,
                recv_id: receiver.id,
                action: ActorAction::Collision(sender.actor_type, CollisionSide::from(direction)),
            }
        }
        MessageType::ChangePosition(change) => {
            ActorMessage::ActorAction {
                send_id: -1,
                recv_id: -1,
                action: ActorAction::ChangePosition(change),
            }
        }
    }
}
