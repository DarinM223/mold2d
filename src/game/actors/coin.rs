use engine::collision::{BoundingBox, Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::{AnimatedSprite, Animation, AnimationData, Renderable, SpriteRectangle};
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

const COIN_VALUE: i32 = 5;

pub struct Coin {
    id: i32,
    rect: SpriteRectangle,
    animation: AnimatedSprite,
}

impl Coin {
    pub fn new(id: i32, position: (i32, i32), renderer: &mut Renderer, fps: f64) -> Coin {
        let anim = Animation::new(AnimationData {
                                      width: 32,
                                      height: 32,
                                      sprites_in_row: 8,
                                      path: "./assets/coin.png",
                                  },
                                  renderer);

        let anims = anim.range(0, 8);

        Coin {
            id: id,
            rect: SpriteRectangle::new(position.0, position.1, 32, 32),
            animation: AnimatedSprite::with_fps(anims, fps),
        }
    }
}

impl Actor for Coin {
    fn on_collision(&mut self, c: &mut Context, o: ActorData, _: CollisionSide) -> ActorAction {
        // Do nothing
        if o.actor_type == ActorType::Player {
            c.score.increment_score("GAME_SCORE", COIN_VALUE);
            return ActorAction::RemoveActor(self.id);
        }

        ActorAction::None
    }

    fn collides_with(&mut self, other_actor: ActorData) -> Option<CollisionSide> {
        self.rect.collides_with(other_actor.rect)
    }

    fn update(&mut self, _context: &mut Context, elapsed: f64) -> ActorAction {
        // Update sprite animation
        self.animation.add_time(elapsed);
        ActorAction::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        self.animation.render(&mut context.renderer, rect);
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: self.id,
            state: 0,
            damage: 0,
            checks_collision: true,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: Some(BoundingBox::Rectangle(self.rect.clone())),
            actor_type: ActorType::Item,
        }
    }
}
