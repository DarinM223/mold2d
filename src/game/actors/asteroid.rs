use engine::context::Context;
use engine::geo_utils;
use engine::sprite::Renderable;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction};
use engine::viewport::Viewport;
use sdl2::rect::Rect;

const ASTEROID_SIDE: u32 = 96;

pub type GameVector = (f64, f64);

spritesheet! {
    name: Asteroid,
    state: AsteroidState,
    path: "./assets/asteroid.png",
    sprite_side: 96,
    sprites_in_row: 21,
    animations: {
        Jumping: 1..2,
        Idle: 1..143
    },
    properties: {
        curr_state: AsteroidState => AsteroidState::Jumping,
        vel: GameVector => (0.0, 0.0),
        acc: GameVector => (0.0, 0.0),
        rect: SpriteRectangle => SpriteRectangle::new(64, 64, ASTEROID_SIDE, ASTEROID_SIDE)
    }
}

impl Actor for Asteroid {
    fn update(&mut self,
              context: &mut Context,
              other_actors: Vec<&mut Box<Actor>>,
              elapsed: f64)
              -> ActorAction {
        // Update sprite animation
        if let Some(animation) = self.animations.get_mut(&self.curr_state) {
            animation.add_time(elapsed);
        }

        if context.events.event_called_once("SPACE") {
            match self.curr_state {
                AsteroidState::Jumping => {}
                AsteroidState::Idle => {
                    self.vel.1 = -10.0;
                    self.curr_state = AsteroidState::Jumping;
                }
            }
        }

        match self.curr_state {
            AsteroidState::Jumping => self.acc.1 += 3.0,
            AsteroidState::Idle => {
                if self.acc.1 > 0.0 {
                    self.acc.1 = 0.0;
                }
            }
        }

        if context.events.event_called("RIGHT") {
            self.acc.0 += 3.0;
        }

        if context.events.event_called("LEFT") {
            self.acc.0 -= 3.0;
        }

        self.vel.0 += self.acc.0 * elapsed;
        self.vel.1 += self.acc.1 * elapsed;

        let new_x = self.rect.x + self.vel.0 as i32;
        let new_y = self.rect.y + self.vel.1 as i32;
        let new_rect = Rect::new_unwrap(new_x, new_y, self.rect.w, self.rect.h);

        let mut collision = false;
        for actor in other_actors {
            if geo_utils::rect_contains_rect(new_rect, actor.bounding_box()) {
                self.curr_state = AsteroidState::Idle;
                collision = true;
                break;
            }
        }

        if !collision {
            self.rect.x += self.vel.0 as i32;
            self.rect.y += self.vel.1 as i32;
        }

        ActorAction::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Follow the asteroid
        viewport.set_position((self.rect.x, self.rect.y));

        // Render sprite animation
        if let Some(animation) = self.animations.get_mut(&self.curr_state) {
            animation.render(&mut context.renderer, rect);
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.rect.x = position.0;
        self.rect.y = position.1;
    }

    fn bounding_box(&self) -> Rect {
        self.rect.to_sdl().unwrap()
    }

    fn position(&self) -> (i32, i32) {
        (self.rect.x, self.rect.y)
    }
}
