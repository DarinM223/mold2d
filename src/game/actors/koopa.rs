use actions::{ActorAction, ActorMessage, ActorType};
use engine::collision;
use engine::collision::{BoundingBox, Collision};
use engine::context::Context;
use engine::sprite::{Animation, AnimationManager, Direction, Renderable, SpriteRectangle};
use engine::vector::Vector2D;
use engine::view::{Actor, ActorData};
use engine::viewport::Viewport;
use sdl2::render::Renderer;

const KOOPA_X_MAXSPEED: f64 = 10.0;
const KOOPA_Y_MAXSPEED: f64 = 15.0;
const KOOPA_ACCELERATION: f64 = 0.18;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum KoopaState {
    Jumping,
    Walking,
    ShellSitting,
    ShellMoving,
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

        let banim = Animation::new(AnimationData {
                                       width: 16,
                                       height: 29,
                                       sprites_in_row: 4,
                                       path: "./assets/koopa.png",
                                   },
                                   renderer);
        let sanim = Animation::new(AnimationData {
                                       width: 16,
                                       height: 16,
                                       sprites_in_row: 4,
                                       path: "./assets/shell.png",
                                   },
                                   renderer);

        let bbox = BoundingBox::Rectangle(SpriteRectangle::new(position.0,
                                                               position.1,
                                                               KOOPA_WIDTH,
                                                               KOOPA_HEIGHT));
        let cbbox = BoundingBox::Rectangle(SpriteRectangle::new(position.0,
                                                                position.1,
                                                                KOOPA_WIDTH,
                                                                KOOPA_WIDTH / 2));

        anims.add((Jumping, Left), banim.range(0, 1), bbox.clone());
        anims.add((Jumping, Right), banim.range(3, 4), bbox.clone());
        anims.add((Walking, Left), banim.range(0, 2), bbox.clone());
        anims.add((Walking, Right), banim.range(2, 4), bbox.clone());
        anims.add((ShellSitting, Left), sanim.range(0, 1), cbbox.clone());
        anims.add((ShellSitting, Right), sanim.range(4, 5), cbbox.clone());
        anims.add((ShellMoving, Left), sanim.range(1, 4), cbbox.clone());

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
    fn handle_message(&mut self, message: &ActorMessage) -> ActorMessage {
        match *message {
            _ => ActorMessage::None,
        }
    }

    fn on_collision(&mut self,
                    _: &mut Context,
                    other: ActorData<ActorType>,
                    side: u8)
                    -> ActorMessage {

        let other_bbox = match other.bounding_box {
            Some(b) => b,
            None => return ActorMessage::None,
        };

        let key = (self.curr_state, self.direction);

        if let Some(ref mut self_bbox) = self.anims.bbox_mut(&key) {
            if side & collision::COLLISION_BOTTOM != 0 && other.actor_type == ActorType::Block {
                if self.curr_state == KoopaState::Jumping {
                    self.curr_state = KoopaState::Walking;
                }

                while self_bbox.collides_with(&other_bbox) == Some(collision::COLLISION_BOTTOM) {
                    self.rect.y -= 1;
                    self_bbox.change_pos(&self.rect);
                }

                self.rect.y += 1;
                self_bbox.change_pos(&self.rect);
                self.grounded = true;
            }
            if side & collision::COLLISION_TOP != 0 && other.actor_type == ActorType::Enemy {
                while self_bbox.collides_with(&other_bbox) == Some(collision::COLLISION_TOP) {
                    self.rect.y += 1;
                    self_bbox.change_pos(&self.rect);
                }
            }
            // if the player hits the koopa at any direction other than top
            if side & 0b1101 != 0 && other.actor_type == ActorType::Player {
                return ActorMessage::ActorAction(other.id, ActorAction::DamageActor(0));
            }
        }

        ActorMessage::None
    }

    fn collides_with(&mut self, other: &ActorData<ActorType>) -> Option<u8> {
        let key = (self.curr_state, self.direction);
        self.anims.collides_with(&key, &other.bounding_box)
    }

    fn update(&mut self, _context: &mut Context, _elapsed: f64) -> ActorMessage {
        let max_y_speed = match self.curr_state {
            KoopaState::Jumping => KOOPA_Y_MAXSPEED,
            KoopaState::Walking | KoopaState::ShellSitting | KoopaState::ShellMoving => 0.,
        };

        let target_speed = Vector2D {
            x: 0.,
            y: max_y_speed,
        };

        self.curr_speed = (KOOPA_ACCELERATION * target_speed) +
                          ((1.0 - KOOPA_ACCELERATION) * self.curr_speed);

        match self.curr_state {
            KoopaState::Jumping => self.rect.y += self.curr_speed.y as i32,
            KoopaState::Walking | KoopaState::ShellSitting | KoopaState::ShellMoving => {}
        };

        self.rect.x += self.curr_speed.x as i32;

        // If actor is no longer grounded, change it to jumping
        if !self.grounded &&
           (self.curr_state == KoopaState::ShellSitting ||
            self.curr_state == KoopaState::ShellMoving ||
            self.curr_state == KoopaState::Walking) {
            self.curr_state = KoopaState::Jumping;
        }

        // Reset grounded to check if there is a bottom collision again
        if self.grounded {
            self.grounded = false;
        }

        ActorMessage::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let key = (self.curr_state, self.direction);
        self.anims.render(&key, &self.rect, viewport, &mut context.renderer, false);
    }

    fn data(&mut self) -> ActorData<ActorType> {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 5,
            collision_filter: 0b1111,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                              .bbox(&(self.curr_state, self.direction))
                              .map(|bb| bb.clone()),
            actor_type: ActorType::Enemy,
        }
    }
}
