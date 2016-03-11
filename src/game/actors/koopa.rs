use engine::collision::{BoundingBox, Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::{Animation, AnimationManager, Direction, Renderable, SpriteRectangle};
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

// TODO(DarinM223): modify this until it works

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum KoopaState {
    Jumping,
    Walking,
    Shell,
}

pub const KOOPA_WIDTH: u32 = 30;
pub const KOOPA_HEIGHT: u32 = 60;

pub struct Koopa {
    id: i32,
    curr_state: KoopaState,
    direction: Direction,
    grounded: bool,
    rect: SpriteRectangle,
    anims: AnimationManager<(KoopaState, Direction)>,
}

impl Koopa {
    pub fn new(id: i32, position: (i32, i32), renderer: &mut Renderer, fps: f64) -> Koopa {
        use engine::sprite::AnimationData;
        use engine::sprite::Direction::*;
        use self::KoopaState::*;

        let mut anims = AnimationManager::new(fps);

        Koopa {
            id: id,
            curr_state: KoopaState::Walking,
            direction: Direction::Left,
            grounded: false,
            rect: SpriteRectangle::new(position.0, position.1, KOOPA_WIDTH, KOOPA_HEIGHT),
            anims: anims,
        }
    }
}

impl Actor for Koopa {
    fn on_collision(&mut self, _: &mut Context, o: ActorData, side: CollisionSide) -> ActorAction {
        ActorAction::None
    }

    fn collides_with(&mut self, other: &ActorData) -> Option<CollisionSide> {
        let key = (self.curr_state, self.direction);
        self.anims.collides_with(&key, &other.bounding_box)
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ActorAction {
        ActorAction::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        let key = (self.curr_state, self.direction);

        // TODO(DarinM223): draws bounding box for debugging purposes only
        if let Some(bounding_box) = self.anims.bbox(&key) {
            match *bounding_box {
                BoundingBox::Rectangle(ref rect) => {
                    context.renderer.set_draw_color(::sdl2::pixels::Color::RGB(230, 230, 230));
                    let (rx, ry) = viewport.relative_point((rect.x, rect.y));
                    let rect = Rect::new_unwrap(rx, ry, rect.w, rect.h);
                    context.renderer.fill_rect(rect);
                }
            }
        }

        // Render sprite animation
        if let Some(animation) = self.anims.anim_mut(&key) {
            animation.render(&mut context.renderer, rect);
        } else {
            println!("Could not find animation for {:?} {:?}",
                     self.curr_state,
                     self.direction);
        }
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: self.id,
            state: self.curr_state as u32,
            damage: 5,
            checks_collision: true,
            rect: self.rect.to_sdl().unwrap(),
            bounding_box: self.anims
                              .bbox(&(self.curr_state, self.direction))
                              .map(|bb| bb.clone()),
            actor_type: ActorType::Enemy,
        }
    }
}
