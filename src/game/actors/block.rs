use engine::context::Context;
use engine::sprite::SpriteRectangle;
use engine::view::{Actor, ActorAction};
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
    fn update(&mut self, _context: &mut Context, _elapsed: f64) -> ActorAction {
        ActorAction::None
    }

    fn render(&mut self, context: &mut Context, viewport: &Viewport, _elapsed: f64) {
        let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
        let rect = Rect::new(rx, ry, self.rect.w, self.rect.h).unwrap().unwrap();

        context.renderer.set_draw_color(Color::RGB(70, 15, 70));
        context.renderer.fill_rect(rect);
    }

    fn set_position(&mut self, position: (i32, i32)) {
        self.rect.x = position.0;
        self.rect.y = position.1;
    }

    fn position(&self) -> (i32, i32) {
        (self.rect.x, self.rect.y)
    }
}
