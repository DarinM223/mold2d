use actions::{ActorAction, ActorData, ActorMessage, ActorType};
use mold2d::{Actor, AnimatedSprite, BoundingBox, Collision, CollisionSide, PositionChange,
             Context, Renderable, Spritesheet, SpritesheetConfig, SpriteRectangle, Viewport};
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
        let anim = Spritesheet::new(SpritesheetConfig {
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
    type Type = ActorType;
    type Message = ActorMessage;

    fn handle_message(&mut self, message: &ActorMessage) -> ActorMessage {
        if let ActorMessage::ActorAction { ref action, .. } = *message {
            match *action {
                // Action when player collides into item
                ActorAction::Collision(ActorType::Player, _) => {
                    // Update score and remove coin
                    ActorMessage::MultipleMessages(vec![
                        Box::new(ActorMessage::UpdateScore(COIN_VALUE)),
                        Box::new(ActorMessage::RemoveActor(self.data().id)),
                    ])
                }
                // Action when an enemy is thrown or kicked into item
                ActorAction::DamageActor(_) => {
                    // Update score and remove coin
                    ActorMessage::MultipleMessages(vec![
                        Box::new(ActorMessage::UpdateScore(COIN_VALUE)),
                        Box::new(ActorMessage::RemoveActor(self.data().id)),
                    ])
                }
                _ => ActorMessage::None,
            }
        } else {
            ActorMessage::None
        }
    }

    fn collides_with(&mut self, other_actor: &ActorData) -> Option<CollisionSide> {
        self.rect.collides_with(other_actor.rect)
    }

    fn update(&mut self, _context: &mut Context, elapsed: f64) -> PositionChange {
        // Update sprite animation
        self.animation.add_time(elapsed);
        PositionChange::new()
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        // Render sprite animation
        self.animation.render(&mut context.renderer, rect);
    }

    fn data(&mut self) -> ActorData {
        ActorData {
            id: self.id,
            state: 0,
            damage: 0,
            collision_filter: 0b1111,
            resolves_collisions: false,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: Some(BoundingBox::Rectangle(self.rect.clone())),
            actor_type: ActorType::Item,
        }
    }
}
