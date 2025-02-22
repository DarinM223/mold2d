/// Macro for easily creating block classes
///
/// ## NOTE:
/// The Type enum must have a Block subtype like this:
/// ```
/// enum Type {
///     ....,
///     Block,
/// }
/// ```
/// and the Message must have a None subtype like this:
/// ```
/// enum Message {
///     ....,
///     None,
/// }
/// ```
///
/// ## Example:
/// ```
/// block! {
///     actor_type: ActorType,
///     actor_message: ActorMessage,
///     blocks: {
///         block {
///             name: GrassBlock, // the name of the block
///             path: "assets/spritesheet.png", // the path of the spritesheet
///             index: 20, // the sprite index inside the spritesheet
///             width: 5, // width of block
///             height: 5, // height of block
///             sprites_in_row: 10, // number of blocks in the spritesheet in a row
///             size: 20, // size of the rendered block
///         }
///
///         block {
///             ....
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! block {
    (
        actor_type: $actor_type:ident,
        actor_message: $actor_message:ident,
        blocks: {
            $(
                block {
                    name: $name:ident,
                    path: $path:expr,
                    index: $index:expr,
                    width: $width:expr,
                    height: $height:expr,
                    sprites_in_row: $sprites_in_row:expr,
                    size: $size:expr,
                    collision_filter: $filter:expr
                }
            )*
        }
    ) => {
        $(
            pub struct $name {
                pub rect: ::mold2d::SpriteRectangle,
                pub sprite: ::mold2d::Sprite,
                index: ::mold2d::ActorIndex,
            }

            impl $name {
                pub fn new(index: ::mold2d::ActorIndex,
                           position: ::mold2d::ActorPosition,
                           renderer: &mut ::sdl2::render::Renderer,
                           _fps: f64)
                           -> $name {
                    let anim_data = ::mold2d::SpritesheetConfig {
                        width: $width,
                        height: $height,
                        sprites_in_row: $sprites_in_row,
                        path: $path,
                    };

                    let anim = ::mold2d::Spritesheet::new(anim_data, renderer);
                    let mut sprite_anims = anim.range($index, $index + 1);
                    let sprite = sprite_anims.pop().unwrap();

                    $name {
                        index,
                        rect: ::mold2d::SpriteRectangle::new(position.0,
                                                             position.1,
                                                             $size,
                                                             $size),
                        sprite: sprite,
                    }
                }
            }

            impl ::mold2d::Actor for $name {
                type Type = $actor_type;
                type Message = $actor_message;

                fn handle_message(&mut self, _: &$actor_message) -> $actor_message {
                    $actor_message::None
                }

                #[allow(unused_imports)]
                fn collides_with(&mut self,
                                 other_actor: &::mold2d::ActorData<$actor_type>)
                                 -> Option<::mold2d::CollisionSide> {
                    use ::mold2d::Collision;
                    self.rect.collides_with(&other_actor.rect)
                }

                fn update(&mut self,
                          _context: &mut ::mold2d::Context,
                          _elapsed: f64)
                          -> ::mold2d::PositionChange {
                    ::mold2d::PositionChange::new()
                }

                #[allow(unused_imports)]
                fn render(&mut self,
                          context: &mut ::mold2d::Context,
                          viewport: &mut ::mold2d::Viewport,
                          _elapsed: f64) -> Result<(), Box<dyn ::std::error::Error>> {
                    use ::mold2d::Renderable;
                    let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
                    let rect = ::sdl2::rect::Rect::new(rx, ry, self.rect.w, self.rect.h);

                    self.sprite.render(&mut context.renderer, rect)
                }

                fn data(&mut self) -> ::mold2d::ActorData<$actor_type> {
                    ::mold2d::ActorData {
                        index: self.index,
                        state: 0 as u32,
                        damage: 0,
                        resolves_collisions: false,
                        collision_filter: $filter,
                        rect: self.rect.to_sdl(),
                        bounding_box: Some(::mold2d::BoundingBox::Rectangle(self.rect.clone())),
                        actor_type: $actor_type::Block,
                    }
                }
            }
        )*
    }
}
