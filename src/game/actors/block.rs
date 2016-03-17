use actions::{ActorMessage, ActorType};
use engine::{Actor, ActorData, Animation, AnimationData, BoundingBox, Collision, Context,
             Renderable, Sprite, SpriteRectangle, Viewport};
use sdl2::rect::Rect;
use sdl2::render::Renderer;

/// Macro for easily creating block classes
/// Example:
/// block! {
///     name: GrassBlock, // the name of the block
///     path: "assets/spritesheet.png", // the path of the spritesheet
///     index: 20, // the sprite index inside the spritesheet
///     width: 5, // width of block
///     height: 5, // height of block
///     sprites_in_row: 10, // number of blocks in the spritesheet in a row
///     size: 20, // size of the rendered block
/// }
#[macro_export]
macro_rules! block {
    (
        name: $name:ident,
        path: $path:expr,
        index: $index:expr,
        width: $width:expr,
        height: $height:expr,
        sprites_in_row: $sprites_in_row:expr,
        size: $size:expr,
        collision_filter: $filter:expr,
    ) => {
        pub struct $name {
            id: i32,
            pub rect: SpriteRectangle,
            pub sprite: Sprite,
        }

        impl $name {
            pub fn new(id: i32,
                       position: (i32, i32),
                       renderer: &mut Renderer,
                       _fps: f64)
                       -> $name {
                let anim_data = AnimationData {
                    width: $width,
                    height: $height,
                    sprites_in_row: $sprites_in_row,
                    path: $path,
                };

                let anim = Animation::new(anim_data, renderer);
                let mut sprite_anims = anim.range($index, $index + 1);
                let sprite = sprite_anims.pop().unwrap();

                $name {
                    id: id,
                    rect: SpriteRectangle::new(position.0,
                                               position.1,
                                               $size,
                                               $size),
                    sprite: sprite,
                }
            }
        }

        impl Actor<ActorType, ActorMessage> for $name {
            fn handle_message(&mut self, _: &ActorMessage) -> ActorMessage {
                ActorMessage::None
            }

            fn on_collision(&mut self,
                            _c: &mut Context,
                            _a: ActorData<ActorType>,
                            _s: u8)
                            -> ActorMessage {
                ActorMessage::None
            }

            #[allow(unused_imports)]
            fn collides_with(&mut self,
                             other_actor: &ActorData<ActorType>)
                             -> Option<u8> {
                self.rect.collides_with(other_actor.rect)
            }

            fn update(&mut self,
                      _context: &mut Context,
                      _elapsed: f64)
                      -> ActorMessage {
                ActorMessage::None
            }

            #[allow(unused_imports)]
            fn render(&mut self,
                      context: &mut Context,
                      viewport: &mut Viewport,
                      _elapsed: f64) {
                let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
                let rect = Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

                self.sprite.render(&mut context.renderer, rect);
            }

            fn data(&mut self) -> ActorData<ActorType> {
                ActorData {
                    id: self.id,
                    state: 0 as u32,
                    damage: 0,
                    collision_filter: $filter,
                    rect: self.rect.to_sdl().unwrap(),
                    bounding_box: Some(BoundingBox::Rectangle(self.rect.clone())),
                    actor_type: ActorType::Block,
                }
            }
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
    collision_filter: 0b1111,
}

block! {
    name: GroundBlockTop,
    path: "assets/tiles.png",
    index: 14,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
    collision_filter: 0b1111,
}

block! {
    name: GroundBlockMid,
    path: "assets/tiles.png",
    index: 21,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
    collision_filter: 0b1110,
}

block! {
    name: StoneBlock,
    path: "assets/tiles.png",
    index: 7,
    width: 80,
    height: 80,
    sprites_in_row: 7,
    size: 40,
    collision_filter: 0b1111,
}
