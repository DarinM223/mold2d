use engine::context::Context;
use engine::physics::PositionUpdater;
use engine::sprite::Renderable;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction};
use engine::viewport::Viewport;
use rand::random;
use sdl2::rect::Rect;

const ASTEROID_SIDE: u32 = 96;

spritesheet! {
    name: Asteroid,
    state: AsteroidState,
    path: "./assets/asteroid.png",
    sprite_side: 96,
    sprites_in_row: 21,
    animations: {
        Jumping: 1..1,
        Idle: 1..143
    },
    properties: {
        curr_state: AsteroidState => AsteroidState::Idle,
        updater: PositionUpdater => {
            let mut updater = PositionUpdater::new();
            updater.add_force("GRAVITY", (0, 9));
            
            updater
        },
        rect: SpriteRectangle => SpriteRectangle::new(64, 64, ASTEROID_SIDE, ASTEROID_SIDE)
    }
}

impl Actor for Asteroid {
    fn update(&mut self,
              context: &mut Context,
              other_actors: Vec<Rect>,
              elapsed: f64)
              -> ActorAction {
        // update sprite animation
        self.animations.get_mut(&self.curr_state).unwrap().add_time(elapsed);

        if context.events.event_called_once("SPACE") {
            match self.curr_state {
                AsteroidState::Jumping => {}
                AsteroidState::Idle => self.updater.add_force("JUMP", (0, -1000)),
            }
        }

        if context.events.event_called("RIGHT") {
            self.updater.add_force("X-Force", (3, 0));
        }

        if context.events.event_called("LEFT") {
            self.updater.add_force("X-Force", (-3, 0));
        }

        self.updater.update(&mut self.rect, other_actors, elapsed);

        self.updater.remove_force("JUMP");
        self.updater.remove_force("X-Force");

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

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new(rx, ry, self.rect.w, self.rect.h).unwrap().unwrap();

        // Follow the asteroid
        viewport.set_position((self.rect.x, self.rect.y));

        self.animations
            .get_mut(&self.curr_state)
            .unwrap()
            .render(&mut context.renderer, rect);
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
