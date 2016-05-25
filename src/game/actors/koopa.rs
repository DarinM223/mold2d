use actions::{ActorAction, ActorMessage, ActorType};
use engine::{Actor, ActorData, Animation, AnimationManager, BoundingBox, CollisionSide, Context,
             Direction, PositionChange, SpriteRectangle, Vector2D, Viewport};
use sdl2::render::Renderer;

const KOOPA_X_MAXSPEED: f64 = 10.0;
const KOOPA_Y_MAXSPEED: f64 = 15.0;
const KOOPA_ACCELERATION: f64 = 0.18;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum KoopaState {
    Jumping,
    Walking,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum KoopaSize {
    Upright,
    Shell,
}

pub const KOOPA_WIDTH: u32 = 30;
pub const KOOPA_HEIGHT: u32 = 60;

pub struct Koopa {
    id: i32,
    curr_state: KoopaState,
    size: KoopaSize,
    direction: Direction,
    grounded: bool,
    curr_speed: Vector2D,
    rect: SpriteRectangle,
    anims: AnimationManager<(KoopaState, KoopaSize, Direction)>,
}

impl Koopa {
    pub fn new(id: i32, position: (i32, i32), renderer: &mut Renderer, fps: f64) -> Koopa {
        use engine::sprite::AnimationData;
        use engine::sprite::Direction::*;
        use self::KoopaSize::*;
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
                                                                KOOPA_HEIGHT / 2));

        anims.add((Jumping, Upright, Left), banim.range(0, 1), bbox.clone());
        anims.add((Jumping, Upright, Right), banim.range(3, 4), bbox.clone());
        anims.add((Walking, Upright, Left), banim.range(0, 2), bbox.clone());
        anims.add((Walking, Upright, Right), banim.range(2, 4), bbox.clone());

        anims.add((Jumping, Shell, Left), sanim.range(0, 1), cbbox.clone());
        anims.add((Jumping, Shell, Right), sanim.range(4, 5), cbbox.clone());
        anims.add((Walking, Shell, Left), sanim.range(0, 1), cbbox.clone());
        anims.add((Walking, Shell, Right), sanim.range(4, 5), cbbox.clone());

        Koopa {
            id: id,
            curr_state: KoopaState::Walking,
            size: KoopaSize::Upright,
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
        use actions::ActorAction::*;

        if let ActorMessage::ActorAction { send_id, ref action, .. } = *message {
            match *action {
                ChangePosition(ref change) => {
                    self.rect.apply_change(change);
                    self.anims.map_bbox_mut(|bbox| bbox.apply_change(&change));
                    ActorMessage::None
                }
                Collision(ActorType::Block, CollisionSide::Bottom) => {
                    if self.curr_state == KoopaState::Jumping {
                        self.curr_state = KoopaState::Walking;
                    }

                    self.grounded = true;
                    ActorMessage::None
                }
                Collision(ActorType::Player, CollisionSide::Top) => {
                    // Turn to shell if upright
                    if self.size != KoopaSize::Shell {
                        let amount: i32 = self.rect.h as i32 / 2;
                        let half_change = PositionChange::new().shrink_height_bot(amount);
                        self.rect.apply_change(&half_change);
                        self.size = KoopaSize::Shell;
                    }

                    ActorMessage::None
                }
                Collision(ActorType::Player, side) if side & 0b1101 != 0 => {
                    // Send damage message to original sender
                    ActorMessage::ActorAction {
                        send_id: self.id,
                        recv_id: send_id,
                        action: ActorAction::DamageActor(0),
                    }
                }
                _ => ActorMessage::None,
            }
        } else {
            ActorMessage::None
        }
    }

    fn collides_with(&mut self, other: &ActorData<ActorType>) -> Option<CollisionSide> {
        let key = (self.curr_state, self.size, self.direction);
        self.anims.collides_with(&key, &other.bounding_box)
    }

    fn update(&mut self, _context: &mut Context, _elapsed: f64) -> PositionChange {
        let max_y_speed = if self.curr_state == KoopaState::Jumping {
            KOOPA_Y_MAXSPEED
        } else {
            0.
        };

        let target_speed = Vector2D {
            x: 0.,
            y: max_y_speed,
        };

        self.curr_speed = (KOOPA_ACCELERATION * target_speed) +
                          ((1.0 - KOOPA_ACCELERATION) * self.curr_speed);

        let mut change = PositionChange::new().left(self.curr_speed.x as i32);
        if self.curr_state == KoopaState::Jumping {
            change = change.down(self.curr_speed.y as i32);
        }

        // If actor is no longer grounded, change it to jumping
        if !self.grounded && self.curr_state != KoopaState::Jumping {
            self.curr_state = KoopaState::Jumping;
        }

        // Reset grounded to check if there is a bottom collision again
        if self.grounded {
            self.grounded = false;
        }

        change
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let key = (self.curr_state, self.size, self.direction);
        self.anims.render(&key, &self.rect, viewport, &mut context.renderer, false);
    }

    fn data(&mut self) -> ActorData<ActorType> {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 5,
            resolves_collisions: true,
            collision_filter: 0b1111,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                .bbox(&(self.curr_state, self.size, self.direction))
                .map(|bb| bb.clone()),
            actor_type: ActorType::Enemy,
        }
    }
}
