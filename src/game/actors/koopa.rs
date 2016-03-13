use actions::{ActorMessage, ActorType};
use engine::collision::{BoundingBox, Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::{Animation, AnimationManager, Direction, Renderable, SpriteRectangle};
use engine::vector::Vector2D;
use engine::view::{Actor, ActorData};
use engine::viewport::Viewport;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

// TODO(DarinM223): modify this until it works

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum KoopaState {
    Jumping,
    Walking,
    Shell,
}

pub const KOOPA_WIDTH: u32 = 30;
pub const KOOPA_HEIGHT: u32 = 60;

pub struct Koopa {
    id: i32,
    curr_state: KoopaState,
    direction: Direction,
    grounded: bool,
    curr_speed: Vector2D,
    rect: SpriteRectangle,
    anims: AnimationManager<(KoopaState, Direction)>,
}

impl Koopa {
    pub fn new(id: i32, position: (i32, i32), renderer: &mut Renderer, fps: f64) -> Koopa {
        use engine::sprite::AnimationData;
        use engine::sprite::Direction::*;
        use self::KoopaState::*;

        let mut anims = AnimationManager::new(fps);

        Koopa {
            id: id,
            curr_state: KoopaState::Walking,
            direction: Direction::Left,
            grounded: false,
            curr_speed: Vector2D { x: 0., y: 0. },
            rect: SpriteRectangle::new(position.0, position.1, KOOPA_WIDTH, KOOPA_HEIGHT),
            anims: anims,
        }
    }
}

impl Actor<ActorType, ActorMessage> for Koopa {
    fn on_collision(&mut self,
                    _: &mut Context,
                    o: ActorData<ActorType>,
                    side: CollisionSide)
                    -> ActorMessage {
        if o.actor_type == ActorType::Player && side == CollisionSide::Top {
            // TODO(DarinM223):
        }

        ActorMessage::None
    }

    fn collides_with(&mut self, other: &ActorData<ActorType>) -> Option<CollisionSide> {
        let key = (self.curr_state, self.direction);
        self.anims.collides_with(&key, &other.bounding_box)
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorMessage {
        ActorMessage::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let key = (self.curr_state, self.direction);
        self.anims.render(&key, &self.rect, viewport, &mut context.renderer, true);
    }

    fn data(&mut self) -> ActorData<ActorType> {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 5,
            checks_collision: true,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                              .bbox(&(self.curr_state, self.direction))
                              .map(|bb| bb.clone()),
            actor_type: ActorType::Enemy,
        }
    }
}
