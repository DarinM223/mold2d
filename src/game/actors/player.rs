use actions::{ActorAction, ActorMessage, ActorType};
use engine::{Actor, ActorData, Animation, AnimationManager, BoundingBox, Collision, CollisionSide,
             Context, Direction, Polygon, PositionChange, Renderable, Segment, SpriteRectangle,
             Vector2D, Viewport};
use sdl2::pixels::Color;
use sdl2::render::Renderer;

const PLAYER_WIDTH: u32 = 30;
const PLAYER_HEIGHT: u32 = 60;
const PLAYER_HALF_HEIGHT: u32 = PLAYER_HEIGHT / 2 + 1;
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
    curr_speed: Vector2D,
    rect: SpriteRectangle,
    anims: AnimationManager<(PlayerSize, PlayerState, Direction)>,
    /// vector debugging parameters
    debug: bool,
    prev_segment: Option<Segment>,
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
                                                                position.1 +
                                                                PLAYER_HALF_HEIGHT as i32,
                                                                PLAYER_WIDTH,
                                                                PLAYER_HALF_HEIGHT));

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
            curr_speed: Vector2D { x: 0., y: 0. },
            rect: SpriteRectangle::new(position.0, position.1, PLAYER_WIDTH, PLAYER_HEIGHT),
            anims: anims,
            debug: true,
            prev_segment: None,
        }
    }
}

impl Actor<ActorType, ActorMessage> for Player {
    fn handle_message(&mut self, message: &ActorMessage) -> ActorMessage {
        if let ActorMessage::ActorAction(_, ref message) = *message {
            match *message {
                ActorAction::DamageActor(_) => {
                    match self.size {
                        PlayerSize::Big | PlayerSize::Crouching => {
                            let amount: i32 = self.rect.h as i32 / 2;
                            let half_change = PositionChange::new().shrink_height_top(amount);
                            self.rect.apply_change(&half_change);
                            self.size = PlayerSize::Small;

                            ActorMessage::None
                        }
                        PlayerSize::Small => ActorMessage::PlayerDied,
                    }
                }
                ActorAction::Collision(_, side) if side == CollisionSide::Top => ActorMessage::None,
                ActorAction::Collision(_, side) if side == CollisionSide::Bottom => {
                    if self.curr_state == PlayerState::Jumping {
                        self.curr_state = PlayerState::Idle;
                    }

                    self.grounded = true;
                    ActorMessage::None
                }
                _ => ActorMessage::None,
            }
        } else {
            ActorMessage::None
        }
    }

    fn collides_with(&mut self, other: &ActorData<ActorType>) -> Option<CollisionSide> {
        let key = (self.size, self.curr_state, self.direction);
        self.anims.collides_with(&key, &other.bounding_box)
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> PositionChange {
        if context.events.event_called("DOWN") {
            if self.size == PlayerSize::Big && self.curr_state != PlayerState::Jumping {
                self.size = PlayerSize::Crouching;
            }
        } else if self.size == PlayerSize::Crouching {
            // TODO(DarinM223): check if big player can fit by raycasting
            self.size = PlayerSize::Big;
        }

        // Jump if space bar is pressed
        if context.events.event_called_once("SPACE") && self.curr_state != PlayerState::Jumping {
            self.curr_speed.y = -70.0;
            self.curr_state = PlayerState::Jumping;
        }

        let max_x_speed = if context.events.event_called("RIGHT") {
            if self.curr_state == PlayerState::Idle {
                self.curr_state = PlayerState::Walking;
            }
            self.direction = Direction::Right;

            -PLAYER_X_MAXSPEED
        } else if context.events.event_called("LEFT") {
            if self.curr_state == PlayerState::Idle {
                self.curr_state = PlayerState::Walking;
            }
            self.direction = Direction::Left;

            PLAYER_X_MAXSPEED
        } else {
            if self.curr_state == PlayerState::Walking {
                self.curr_state = PlayerState::Idle;
            }

            0.0
        };

        let max_y_speed = if self.curr_state == PlayerState::Jumping {
            PLAYER_Y_MAXSPEED
        } else {
            0.
        };

        let target_speed = Vector2D {
            x: max_x_speed,
            y: max_y_speed,
        };

        self.curr_speed = (PLAYER_ACCELERATION * target_speed) +
                          ((1.0 - PLAYER_ACCELERATION) * self.curr_speed);

        // Apply position change
        let mut change = PositionChange::new().left(self.curr_speed.x as i32);
        if self.curr_state == PlayerState::Jumping {
            change = change.down(self.curr_speed.y as i32);
        }

        // If not grounded, change to jumping
        if !self.grounded && self.curr_state != PlayerState::Jumping {
            self.curr_state = PlayerState::Jumping;
        }

        // Reset grounded collision
        if self.grounded {
            self.grounded = false;
        }

        // Update sprite animation
        let key = (self.size, self.curr_state, self.direction);
        self.anims.add_time(&key, elapsed);

        self.prev_segment = Some(Segment {
            point: (self.data().rect.x() as f64, self.data().rect.y() as f64),
            vector: change.to_vector(),
        });

        change
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        // renders position change in debug mode
        if self.debug {
            let data = self.data();
            if let Some(ref mut segment) = self.prev_segment {
                segment.render(Color::RGB(0, 0, 0), viewport, &mut context.renderer);
            }
            for side in data.rect.sides() {
                side.render(Color::RGB(0, 255, 0), viewport, &mut context.renderer);
            }
        }

        let key = (self.size, self.curr_state, self.direction);
        self.anims.render(&key, &self.rect, viewport, &mut context.renderer, false);
    }

    fn data(&mut self) -> ActorData<ActorType> {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 0,
            collision_filter: 0b1111,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                              .bbox(&(self.size, self.curr_state, self.direction))
                              .map(|bb| bb.clone()),
            actor_type: ActorType::Player,
        }
    }

    fn change_pos(&mut self, change: &PositionChange) {
        self.rect.apply_change(&change);
        self.anims.map_bbox_mut(|bbox| bbox.apply_change(&change));
    }
}
