use engine::collision::CollisionSide;
use engine::context::Context;
use engine::sprite::{AnimatedSprite, Animation, AnimationData, Renderable, SpriteRectangle};
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

pub struct Coin {
    rect: SpriteRectangle,
    animation: AnimatedSprite,
}

impl Coin {
    pub fn new(renderer: &mut Renderer, fps: f64) -> Coin {
        let anim_data = AnimationData {
            width: 32,
            height: 32,
            sprites_in_row: 8,
            path: "./assets/coin.png",
        };
        let anim = Animation::new(anim_data, renderer);
        let anims = anim.range(0, 8);

        Coin {
            rect: SpriteRectangle::new(64, 64, 32, 32),
            animation: AnimatedSprite::with_fps(anims, fps),
        }
    }
}

impl Actor for Coin {
    fn on_collision(&mut self, _other_actor: ActorData, _side: CollisionSide) {
        // Do nothing
    }

    fn update(&mut self, _context: &mut Context, elapsed: f64) -> Vec<ActorAction> {
        // Update sprite animation
        self.animation.add_time(elapsed);
        vec![]
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        self.animation.render(&mut context.renderer, rect);
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: 0,
            state: 0,
            damage: 0,
            checks_collision: false,
            rect: self.rect.to_sdl().unwrap(),
            actor_type: ActorType::Item,
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.rect.x = position.0;
        self.rect.y = position.1;
    }
}
