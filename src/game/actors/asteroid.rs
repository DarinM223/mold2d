use engine::context::Context;
use engine::sprite::Renderable;
use engine::sprite::SpriteRectangle;
use engine::view::Actor;

const ASTEROID_SIDE: u32 = 96;

spritesheet! {
    name: Asteroid,
    state: AsteroidState,
    path: "./assets/asteroid.png",
    sprite_side: 96,
    sprites_in_row: 21,
    animations: {
        Spinning: 1..143
    },
    properties: {
        curr_state: AsteroidState => AsteroidState::Spinning,
        rect: SpriteRectangle => SpriteRectangle::new(64, 64, ASTEROID_SIDE, ASTEROID_SIDE),
        vel: f64 => 0.0
    }
}

impl Actor for Asteroid {
    fn update(&mut self, context: &mut Context, elapsed: f64) {
        if context.events.event_called("RIGHT") {
            self.vel += 9.0;
        } else if context.events.event_called("LEFT") {
            self.vel -= 9.0;
        } else {
            if self.vel > 0.0 {
                self.vel -= 9.0;
            } else if self.vel < 0.0 {
                self.vel += 9.0;
            }
        }

        self.rect.x += (self.vel * elapsed) as i32;

        // update sprite animation
        self.animations.get_mut(&self.curr_state).unwrap().add_time(elapsed);
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) {
        self.animations
            .get_mut(&self.curr_state)
            .unwrap()
            .render(&mut context.renderer, self.rect.to_sdl().unwrap());
    }
}
