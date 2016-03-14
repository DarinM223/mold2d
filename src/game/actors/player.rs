use actions::{ActorAction, ActorMessage, ActorType};
use engine::collision::{BoundingBox, Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::{Animation, AnimationManager, Direction, Renderable, SpriteRectangle};
use engine::vector::Vector2D;
use engine::view::{Actor, ActorData};
use engine::viewport::Viewport;
use sdl2::render::Renderer;

const PLAYER_WIDTH: u32 = 30;
const PLAYER_HEIGHT: u32 = 60;
const PLAYER_X_MAXSPEED: f64 = 15.0;
const PLAYER_Y_MAXSPEED: f64 = 15.0;
const PLAYER_ACCELERATION: f64 = 0.18;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PlayerState {
    Jumping,
    Walking,
    Idle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PlayerSize {
    Big,
    Small,
    Crouching,
}

pub struct Player {
    id: i32,
    curr_state: PlayerState,
    direction: Direction,
    size: PlayerSize,
    grounded: bool,
    hit_ceiling: bool,
    curr_speed: Vector2D,
    rect: SpriteRectangle,
    anims: AnimationManager<(PlayerSize, PlayerState, Direction)>,
}

impl Player {
    pub fn new(id: i32, position: (i32, i32), renderer: &mut Renderer, fps: f64) -> Player {
        use engine::sprite::AnimationData;
        use engine::sprite::Direction::*;
        use self::PlayerSize::*;
        use self::PlayerState::*;

        let mut anims = AnimationManager::new(fps);

        let banim = Animation::new(AnimationData {
                                       width: 16,
                                       height: 32,
                                       sprites_in_row: 4,
                                       path: "./assets/mario-big.png",
                                   },
                                   renderer);
        let sanim = Animation::new(AnimationData {
                                       width: 16,
                                       height: 16,
                                       sprites_in_row: 4,
                                       path: "./assets/mario-small.png",
                                   },
                                   renderer);

        let bbox = BoundingBox::Rectangle(SpriteRectangle::new(position.0,
                                                               position.1,
                                                               PLAYER_WIDTH,
                                                               PLAYER_HEIGHT));
        let cbbox = BoundingBox::Rectangle(SpriteRectangle::new(position.0,
                                                                position.1,
                                                                PLAYER_WIDTH,
                                                                PLAYER_HEIGHT / 2 + 10));

        anims.add((Big, Idle, Left), banim.range(1, 2), bbox.clone());
        anims.add((Big, Idle, Right), banim.range(12, 13), bbox.clone());
        anims.add((Big, Walking, Left), banim.range(5, 8), bbox.clone());
        anims.add((Big, Walking, Right), banim.range(13, 16), bbox.clone());
        anims.add((Big, Jumping, Left), banim.range(3, 4), bbox.clone());
        anims.add((Big, Jumping, Right), banim.range(11, 12), bbox.clone());

        anims.add((Small, Idle, Left), sanim.range(0, 1), cbbox.clone());
        anims.add((Small, Idle, Right), sanim.range(8, 9), cbbox.clone());
        anims.add((Small, Walking, Left), sanim.range(1, 4), cbbox.clone());
        anims.add((Small, Walking, Right), sanim.range(9, 12), cbbox.clone());
        anims.add((Small, Jumping, Left), sanim.range(4, 5), cbbox.clone());
        anims.add((Small, Jumping, Right), sanim.range(12, 13), cbbox.clone());

        anims.add((Crouching, Idle, Left), banim.range(2, 3), cbbox.clone());
        anims.add((Crouching, Idle, Right), banim.range(10, 11), cbbox.clone());
        anims.add((Crouching, Jumping, Left), banim.range(2, 3), cbbox.clone());
        anims.add((Crouching, Jumping, Right),
                  banim.range(10, 11),
                  cbbox.clone());
        anims.add((Crouching, Walking, Left), banim.range(2, 3), cbbox.clone());
        anims.add((Crouching, Walking, Right),
                  banim.range(10, 11),
                  cbbox.clone());

        Player {
            id: id,
            curr_state: PlayerState::Jumping,
            direction: Direction::Right,
            size: PlayerSize::Big,
            grounded: false,
            hit_ceiling: false,
            curr_speed: Vector2D { x: 0., y: 0. },
            rect: SpriteRectangle::new(position.0, position.1, PLAYER_WIDTH, PLAYER_HEIGHT),
            anims: anims,
        }
    }
}

impl Actor<ActorType, ActorMessage> for Player {
    fn handle_message(&mut self, message: &ActorMessage) -> ActorMessage {
        match *message {
            ActorMessage::ActorAction(_, ref message) => {
                match *message {
                    ActorAction::DamageActor(_) => {
                        match self.size {
                            PlayerSize::Big |
                            PlayerSize::Crouching => {
                                self.rect.h /= 2;
                                self.size = PlayerSize::Small;

                                ActorMessage::None
                            }
                            PlayerSize::Small => ActorMessage::PlayerDied,
                        }
                    }
                }
            }
            _ => ActorMessage::None,
        }
    }

    fn on_collision(&mut self,
                    _: &mut Context,
                    other: ActorData<ActorType>,
                    side: CollisionSide)
                    -> ActorMessage {
        let other_bbox = match other.bounding_box {
            Some(b) => b,
            None => return ActorMessage::None,
        };

        let key = (self.size, self.curr_state, self.direction);

        if let Some(ref mut self_bbox) = self.anims.bbox_mut(&key) {
            match side {
                CollisionSide::Left => {
                    while self_bbox.collides_with(&other_bbox) == Some(CollisionSide::Left) {
                        self.rect.x -= 1;
                        self_bbox.change_pos(&self.rect);
                    }
                }
                CollisionSide::Right => {
                    while self_bbox.collides_with(&other_bbox) == Some(CollisionSide::Right) {
                        self.rect.x += 1;
                        self_bbox.change_pos(&self.rect);
                    }
                }
                CollisionSide::Top => {
                    while self_bbox.collides_with(&other_bbox) == Some(CollisionSide::Top) {
                        self.rect.y += 1;
                        self_bbox.change_pos(&self.rect);
                    }

                    self.curr_speed.y = 0.;
                    self.hit_ceiling = true;
                }
                CollisionSide::Bottom => {
                    if self.curr_state == PlayerState::Jumping {
                        self.curr_state = PlayerState::Idle;
                    }

                    while self_bbox.collides_with(&other_bbox) == Some(CollisionSide::Bottom) {
                        self.rect.y -= 1;
                        self_bbox.change_pos(&self.rect);
                    }

                    self.rect.y += 1;
                    self_bbox.change_pos(&self.rect);
                    self.grounded = true;
                }
            }
        }

        ActorMessage::None
    }

    fn collides_with(&mut self, other: &ActorData<ActorType>) -> Option<CollisionSide> {
        let key = (self.size, self.curr_state, self.direction);
        self.anims.collides_with(&key, &other.bounding_box)
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorMessage {
        let max_y_speed = match self.curr_state {
            PlayerState::Jumping => PLAYER_Y_MAXSPEED,
            PlayerState::Idle | PlayerState::Walking => 0.0,
        };

        if context.events.event_called("DOWN") {
            if self.size == PlayerSize::Big &&
               (self.curr_state == PlayerState::Walking || self.curr_state == PlayerState::Idle) {
                self.size = PlayerSize::Crouching;
            }
        } else if self.size == PlayerSize::Crouching && !self.hit_ceiling {
            self.size = PlayerSize::Big;
        }

        let max_x_speed;
        if context.events.event_called("RIGHT") {
            if self.curr_state == PlayerState::Idle {
                self.curr_state = PlayerState::Walking;
            }
            self.direction = Direction::Right;
            max_x_speed = PLAYER_X_MAXSPEED;
        } else if context.events.event_called("LEFT") {
            if self.curr_state == PlayerState::Idle {
                self.curr_state = PlayerState::Walking;
            }
            self.direction = Direction::Left;
            max_x_speed = -PLAYER_X_MAXSPEED;
        } else {
            if self.curr_state == PlayerState::Walking {
                self.curr_state = PlayerState::Idle;
            }
            max_x_speed = 0.0;
        }

        if context.events.event_called_once("SPACE") && !self.hit_ceiling {
            match self.curr_state {
                PlayerState::Jumping => {}
                PlayerState::Idle | PlayerState::Walking => {
                    self.curr_speed.y = -70.0;
                    self.curr_state = PlayerState::Jumping;
                }
            }
        }

        let target_speed = Vector2D {
            x: max_x_speed,
            y: max_y_speed,
        };

        self.curr_speed = (PLAYER_ACCELERATION * target_speed) +
                          ((1.0 - PLAYER_ACCELERATION) * self.curr_speed);

        // Don't allow jumping if player already collides with the ceiling
        if self.hit_ceiling {
            self.hit_ceiling = false;
            if self.curr_speed.y < 0. {
                self.curr_speed.y = 0.;
            }
        }

        match self.curr_state {
            PlayerState::Jumping => self.rect.y += self.curr_speed.y as i32,
            PlayerState::Idle | PlayerState::Walking => {}
        }

        self.rect.x += self.curr_speed.x as i32;

        // If actor is no longer grounded, change it to jumping
        if !self.grounded &&
           (self.curr_state == PlayerState::Idle || self.curr_state == PlayerState::Walking) {
            self.curr_state = PlayerState::Jumping;
        }

        // Reset grounded to check if there is a bottom collision again
        if self.grounded {
            self.grounded = false;
        }

        // Update sprite animation
        let key = (self.size, self.curr_state, self.direction);
        self.anims.add_time(&key, elapsed);
        self.anims.change_pos(&key, &self.rect);

        ActorMessage::SetViewport(self.rect.x, self.rect.y)
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let key = (self.size, self.curr_state, self.direction);
        self.anims.render(&key, &self.rect, viewport, &mut context.renderer, false);
    }

    fn data(&mut self) -> ActorData<ActorType> {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 0,
            checks_collision: true,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                              .bbox(&(self.size, self.curr_state, self.direction))
                              .map(|bb| bb.clone()),
            actor_type: ActorType::Player,
        }
    }
}
