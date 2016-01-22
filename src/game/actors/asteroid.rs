use engine::context::Context;
use engine::sprite::Renderable;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction};
use rand::random;

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
    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorAction {
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

        if context.events.event_called_once("UP") {
            let mut new_asteroid = Asteroid::new(&mut context.renderer, 60 as f64);
            let max_width = context.window.width - 100;
            let max_height = context.window.height - 100;

            new_asteroid.rect.x = (random::<u32>() % max_width) as i32 + 1;
            new_asteroid.rect.y = (random::<u32>() % max_height) as i32 + 1;

            ActorAction::AddActor(Box::new(new_asteroid))
        } else {
            ActorAction::None
        }
    }

    fn render(&mut self, context: &mut Context, elapsed: f64) {
        self.animations
            .get_mut(&self.curr_state)
            .unwrap()
            .render(&mut context.renderer, self.rect.to_sdl().unwrap());
    }
}
