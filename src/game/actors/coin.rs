use actions::{ActorAction, ActorMessage, ActorType};
use engine::{Actor, ActorData, AnimatedSprite, Animation, AnimationData, BoundingBox, Collision,
             Context, Renderable, SpriteRectangle, Viewport};
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

impl Actor<ActorType, ActorMessage> for Coin {
    fn handle_message(&mut self, message: &ActorMessage) -> ActorMessage {
        if let ActorMessage::ActorAction(_, ref message) = *message {
            match *message {
                ActorAction::Collision(actor_type, _) if actor_type == ActorType::Player => {
                    ActorMessage::UpdateScore(COIN_VALUE)
                }
                _ => ActorMessage::None,
            }
        } else {
            ActorMessage::None
        }
    }

    fn collides_with(&mut self, other_actor: &ActorData<ActorType>) -> Option<u8> {
        self.rect.collides_with(other_actor.rect)
    }

    fn update(&mut self, _context: &mut Context, elapsed: f64) -> ActorMessage {
        // Update sprite animation
        self.animation.add_time(elapsed);
        ActorMessage::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        self.animation.render(&mut context.renderer, rect);
    }

    fn data(&mut self) -> ActorData<ActorType> {
        ActorData {
            id: self.id,
            state: 0,
            damage: 0,
            collision_filter: 0b1111,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: Some(BoundingBox::Rectangle(self.rect.clone())),
            actor_type: ActorType::Item,
        }
    }
}
