use engine::collision::{BoundingBox, Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::Renderable;
use engine::sprite::{Animation, AnimationData, AnimationManager, SpriteRectangle};
use engine::vector::Vector2D;
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

const PLAYER_SIDE: u32 = 40;
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
pub enum WalkDirection {
    Left,
    Right,
}

pub struct Player {
    id: i32,
    curr_state: PlayerState,
    direction: WalkDirection,
    grounded: bool,
    curr_speed: Vector2D,
    rect: SpriteRectangle,
    anims: AnimationManager<(PlayerState, WalkDirection)>,
}

impl Player {
    pub fn new(id: i32, position: (i32, i32), renderer: &mut Renderer, fps: f64) -> Player {
        use self::PlayerState::*;
        use self::WalkDirection::*;

        let mut anims = AnimationManager::new(fps);

        let anim = Animation::new(AnimationData {
                                      width: 16,
                                      height: 16,
                                      sprites_in_row: 4,
                                      path: "./assets/mario-small.png",
                                  },
                                  renderer);

        let bounding_box = BoundingBox::Rectangle(SpriteRectangle::new(position.0,
                                                                       position.1,
                                                                       PLAYER_SIDE,
                                                                       PLAYER_SIDE));

        anims.add((Idle, Left), anim.range(0, 1), bounding_box.clone());
        anims.add((Idle, Right), anim.range(8, 9), bounding_box.clone());
        anims.add((Walking, Left), anim.range(1, 4), bounding_box.clone());
        anims.add((Walking, Right), anim.range(9, 12), bounding_box.clone());
        anims.add((Jumping, Left), anim.range(4, 5), bounding_box.clone());
        anims.add((Jumping, Right), anim.range(12, 13), bounding_box.clone());

        Player {
            id: id,
            curr_state: PlayerState::Jumping,
            direction: WalkDirection::Right,
            grounded: false,
            curr_speed: Vector2D { x: 0., y: 0. },
            rect: SpriteRectangle::new(position.0, position.1, PLAYER_SIDE, PLAYER_SIDE),
            anims: anims,
        }
    }
}

impl Actor for Player {
    fn on_collision(&mut self, _: &mut Context, o: ActorData, side: CollisionSide) -> ActorAction {
        if o.actor_type == ActorType::Item {
            return ActorAction::None;
        }

        match side {
            CollisionSide::Left => {
                while self.rect.collides_with(o.rect) == Some(CollisionSide::Left) {
                    self.rect.x -= 1;
                }
            }
            CollisionSide::Right => {
                while self.rect.collides_with(o.rect) == Some(CollisionSide::Right) {
                    self.rect.x += 1;
                }
            }
            CollisionSide::Top => {}
            CollisionSide::Bottom => {
                if self.curr_state == PlayerState::Jumping {
                    self.curr_state = PlayerState::Idle;
                }

                while self.rect.collides_with(o.rect) == Some(CollisionSide::Bottom) {
                    self.rect.y -= 1;
                }

                self.rect.y += 1;
                self.grounded = true;
            }
        }

        ActorAction::None
    }

    fn collides_with(&mut self, other_actor: ActorData) -> Option<CollisionSide> {
        if let Some(bounding_box) = self.anims.bbox(&(self.curr_state, self.direction)) {
            if let Some(other_box) = other_actor.bounding_box {
                return bounding_box.collides_with(other_box);
            }
        }

        None
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorAction {
        let max_y_speed = match self.curr_state {
            PlayerState::Jumping => PLAYER_Y_MAXSPEED,
            PlayerState::Idle | PlayerState::Walking => 0.0,
        };

        let max_x_speed;
        if context.events.event_called("RIGHT") {
            if self.curr_state == PlayerState::Idle {
                self.curr_state = PlayerState::Walking;
            }
            self.direction = WalkDirection::Right;
            max_x_speed = PLAYER_X_MAXSPEED;
        } else if context.events.event_called("LEFT") {
            if self.curr_state == PlayerState::Idle {
                self.curr_state = PlayerState::Walking;
            }
            self.direction = WalkDirection::Left;
            max_x_speed = -PLAYER_X_MAXSPEED;
        } else {
            if self.curr_state == PlayerState::Walking {
                self.curr_state = PlayerState::Idle;
            }
            max_x_speed = 0.0;
        }

        if context.events.event_called_once("SPACE") {
            match self.curr_state {
                PlayerState::Jumping => {}
                PlayerState::Idle | PlayerState::Walking => {
                    self.curr_speed.y = -100.0;
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
        if let Some(animation) = self.anims.anim_mut(&(self.curr_state, self.direction)) {
            animation.add_time(elapsed);
        }

        // Update sprite bounding box
        if let Some(bounding_box) = self.anims.bbox_mut(&(self.curr_state, self.direction)) {
            bounding_box.change_pos((self.rect.x, self.rect.y));
        }

        ActorAction::SetViewport(self.rect.x, self.rect.y)
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        if let Some(animation) = self.anims.anim_mut(&(self.curr_state, self.direction)) {
            animation.render(&mut context.renderer, rect);
        } else {
            println!("Could not find animation for {:?} {:?}",
                     self.curr_state,
                     self.direction);
        }
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 0,
            checks_collision: true,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                              .bbox(&(self.curr_state, self.direction))
                              .map(|bb| bb.clone()),
            actor_type: ActorType::Player,
        }
    }
}
