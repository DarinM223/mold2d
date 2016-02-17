use engine::collision::{Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::Renderable;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::rect::Rect;

spritesheet! {
    name: Coin,
    state: CoinState,
    path: "./assets/coin.png",
    sprite_width: 32,
    sprite_height: 32,
    sprites_in_row: 8,
    animations: {
        Idle: 0..8
    },
    properties: {
        rect: SpriteRectangle => SpriteRectangle::new(64, 64, 32, 32)
    }
}

impl Actor for Coin {
    fn update(&mut self,
              context: &mut Context,
              other_actors: &Vec<ActorData>,
              elapsed: f64)
              -> Vec<ActorAction> {

        // Update sprite animation
        if let Some(animation) = self.animations.get_mut(&CoinState::Idle) {
            animation.add_time(elapsed);
        }

        vec![]
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        if let Some(animation) = self.animations.get_mut(&CoinState::Idle) {
            animation.render(&mut context.renderer, rect);
        }
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: 0,
            state: 0,
            damage: 0,
            rect: self.rect.to_sdl().unwrap(),
            actor_type: ActorType::Item,
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.rect.x = position.0;
        self.rect.y = position.1;
    }
}
