use engine::collision::CollisionSide;
use engine::context::Context;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction, ActorData, ActorType};
use engine::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;

const BLOCK_SIZE: u32 = 40;

/// Prototype struct to test rendering blocks
/// TODO: Remove after grid layout system is completed
pub struct Block {
    pub rect: SpriteRectangle,
}

impl Block {
    pub fn new(_renderer: &mut Renderer, _fps: f64) -> Block {
        Block { rect: SpriteRectangle::new(0, 0, BLOCK_SIZE, BLOCK_SIZE) }
    }
}

impl Actor for Block {
    fn on_collision(&mut self, _other_actor: ActorData, _side: CollisionSide) {
        // Do nothing
    }

    fn update(&mut self, _context: &mut Context, _elapsed: f64) -> Vec<ActorAction> {
        vec![]
    }

    fn render(&mut self, context: &mut Context, viewport: &mut Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

        context.renderer.set_draw_color(Color::RGB(85, 107, 47));
        context.renderer.fill_rect(rect);
    }

    fn data(&self) -> ActorData {
        ActorData {
            id: 0,
            state: 0 as u32,
            damage: 0,
            checks_collision: false,
            rect: self.rect.to_sdl().unwrap(),
            actor_type: ActorType::Block,
        }
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.rect.x = position.0;
        self.rect.y = position.1;
    }
}
