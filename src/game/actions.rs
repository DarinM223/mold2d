use actors::block::{GroundBlockMid, GroundBlockTop, StartBlock, StoneBlock};
use actors::coin::Coin;
use actors::koopa::Koopa;
use actors::player::Player;
use engine::{Actor, ActorFromToken};
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
