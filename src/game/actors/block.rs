use engine::collision::{Collision, CollisionSide};
use engine::context::Context;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

const BLOCK_SIZE: u32 = 40;

/// Prototype struct to test rendering blocks
pub struct Block {
    id: i32,
    pub rect: SpriteRectangle,
}

impl Block {
    pub fn new(id: i32, position: (i32, i32), _renderer: &mut Renderer, _fps: f64) -> Block {
        Block {
            id: id,
            rect: SpriteRectangle::new(position.0, position.1, BLOCK_SIZE, BLOCK_SIZE),
        }
    }
}

impl Actor for Block {
    fn on_collision(&mut self, _c: &mut Context, _a: ActorData, _s: CollisionSide) -> ActorAction {
        ActorAction::None
    }

    fn collides_with(&mut self, other_actor: ActorData) -> Option<CollisionSide> {
        self.rect.collides_with(other_actor.rect)
    }

    fn update(&mut self, _context: &mut Context, _elapsed: f64) -> ActorAction {
        ActorAction::None
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        context.renderer.set_draw_color(Color::RGB(85, 107, 47));
        context.renderer.fill_rect(rect);
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: self.id,
            state: 0 as u32,
            damage: 0,
            checks_collision: false,
            rect: self.rect.to_sdl().unwrap(),
            actor_type: ActorType::Block,
        }
    }
}

block! {
    name: StartBlock,
    path: "assets/tiles.png",
    index: 0,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
}

block! {
    name: GroundBlockTop,
    path: "assets/tiles.png",
    index: 14,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
}

block! {
    name: GroundBlockMid,
    path: "assets/tiles.png",
    index: 21,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
}

block! {
    name: StoneBlock,
    path: "assets/tiles.png",
    index: 7,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
}
