use engine::collision::{Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::Renderable;
use engine::sprite::{AnimatedSprite, Animation, AnimationData, SpriteRectangle};
use engine::vector::Vector2D;
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use std::collections::HashMap;

const ASTEROID_SIDE: u32 = 96;
const ASTEROID_X_MAXSPEED: f64 = 10.0;
const ASTEROID_Y_MAXSPEED: f64 = 15.0;
const ASTEROID_ACCELERATION: f64 = 0.2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AsteroidState {
    Jumping,
    Idle,
}

pub struct Asteroid {
    curr_state: AsteroidState,
    grounded: bool,
    curr_speed: Vector2D,
    rect: SpriteRectangle,
    animations: HashMap<AsteroidState, AnimatedSprite>,
}

impl Asteroid {
    pub fn new(renderer: &mut Renderer, fps: f64) -> Asteroid {
        let mut animations = HashMap::new();

        let anim_data = AnimationData {
            width: 96,
            height: 96,
            sprites_in_row: 21,
            path: "./assets/asteroid.png",
        };
        let anim = Animation::new(anim_data, renderer);
        let jumping_anims = anim.range(0, 1);
        let idle_anims = anim.range(0, 143);

        animations.insert(AsteroidState::Jumping,
                          AnimatedSprite::with_fps(jumping_anims, fps));
        animations.insert(AsteroidState::Idle,
                          AnimatedSprite::with_fps(idle_anims, fps));

        Asteroid {
            curr_state: AsteroidState::Jumping,
            grounded: false,
            curr_speed: Vector2D { x: 0., y: 0. },
            rect: SpriteRectangle::new(64, 64, ASTEROID_SIDE, ASTEROID_SIDE),
            animations: animations,
        }
    }
}

impl Actor for Asteroid {
    fn on_collision(&mut self, other_actor: ActorData, side: CollisionSide) {
        match side {
            CollisionSide::Left => {
                while self.rect.collides_with(other_actor.rect) == Some(CollisionSide::Left) {
                    self.rect.x -= 1;
                }
            }
            CollisionSide::Right => {
                while self.rect.collides_with(other_actor.rect) == Some(CollisionSide::Right) {
                    self.rect.x += 1;
                }
            }
            CollisionSide::Top => {}
            CollisionSide::Bottom => {
                if self.curr_state == AsteroidState::Jumping {
                    self.curr_state = AsteroidState::Idle;
                }

                while self.rect.collides_with(other_actor.rect) == Some(CollisionSide::Bottom) {
                    self.rect.y -= 1;
                }

                self.rect.y += 1;
                self.grounded = true;
            }
        }

    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> Vec<ActorAction> {
        let mut actions = Vec::new();

        let max_y_speed = match self.curr_state {
            AsteroidState::Jumping => ASTEROID_Y_MAXSPEED,
            AsteroidState::Idle => 0.0,
        };

        let max_x_speed = if context.events.event_called("RIGHT") {
            ASTEROID_X_MAXSPEED
        } else if context.events.event_called("LEFT") {
            -ASTEROID_X_MAXSPEED
        } else {
            0.0
        };

        if context.events.event_called_once("SPACE") {
            match self.curr_state {
                AsteroidState::Jumping => {}
                AsteroidState::Idle => {
                    self.curr_speed.y = -100.0;
                    self.curr_state = AsteroidState::Jumping;
                }
            }
        }

        let target_speed = Vector2D {
            x: max_x_speed,
            y: max_y_speed,
        };

        self.curr_speed = (ASTEROID_ACCELERATION * target_speed) +
                          ((1.0 - ASTEROID_ACCELERATION) * self.curr_speed);

        match self.curr_state {
            AsteroidState::Jumping => self.rect.y += self.curr_speed.y as i32,
            AsteroidState::Idle => {}
        }

        self.rect.x += self.curr_speed.x as i32;

        // If actor is no longer grounded, change it to jumping
        if !self.grounded && self.curr_state == AsteroidState::Idle {
            self.curr_state = AsteroidState::Jumping;
        }

        // Reset grounded to check if there is a bottom collision again
        if self.grounded {
            self.grounded = false;
        }

        actions.push(ActorAction::SetViewport(self.rect.x, self.rect.y));

        // Update sprite animation
        if let Some(animation) = self.animations.get_mut(&self.curr_state) {
            animation.add_time(elapsed);
        }

        actions
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        if let Some(animation) = self.animations.get_mut(&self.curr_state) {
            animation.render(&mut context.renderer, rect);
        }
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: 0,
            state: self.curr_state as u32,
            damage: 0,
            checks_collision: true,
            rect: self.rect.to_sdl().unwrap(),
            actor_type: ActorType::Player,
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.rect.x = position.0;
        self.rect.y = position.1;
    }
}
