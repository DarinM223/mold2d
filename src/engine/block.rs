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
                pub rect: ::engine::SpriteRectangle,
                pub sprite: ::engine::Sprite,
                id: i32,
            }

            impl $name {
                pub fn new(id: i32,
                           position: (i32, i32),
                           renderer: &mut ::sdl2::render::Renderer,
                           _fps: f64)
                           -> $name {
                    let anim_data = ::engine::SpritesheetConfig {
                        width: $width,
                        height: $height,
                        sprites_in_row: $sprites_in_row,
                        path: $path,
                    };

                    let anim = ::engine::Spritesheet::new(anim_data, renderer);
                    let mut sprite_anims = anim.range($index, $index + 1);
                    let sprite = sprite_anims.pop().unwrap();

                    $name {
                        id: id,
                        rect: ::engine::SpriteRectangle::new(position.0,
                                                             position.1,
                                                             $size,
                                                             $size),
                        sprite: sprite,
                    }
                }
            }

            impl ::engine::Actor<$actor_type, $actor_message> for $name {
                fn handle_message(&mut self, _: &$actor_message) -> $actor_message {
                    $actor_message::None
                }

                #[allow(unused_imports)]
                fn collides_with(&mut self,
                                 other_actor: &::engine::ActorData<$actor_type>)
                                 -> Option<::engine::CollisionSide> {
                    use ::engine::Collision;
                    self.rect.collides_with(other_actor.rect)
                }

                fn update(&mut self,
                          _context: &mut ::engine::Context,
                          _elapsed: f64)
                          -> ::engine::PositionChange {
                    ::engine::PositionChange::new()
                }

                #[allow(unused_imports)]
                fn render(&mut self,
                          context: &mut ::engine::Context,
                          viewport: &mut ::engine::Viewport,
                          _elapsed: f64) {
                    use ::engine::Renderable;
                    let (rx, ry) = viewport.relative_point((self.rect.x, self.rect.y));
                    let rect = ::sdl2::rect::Rect::new_unwrap(rx, ry, self.rect.w, self.rect.h);

                    self.sprite.render(&mut context.renderer, rect);
                }

                fn data(&mut self) -> ::engine::ActorData<$actor_type> {
                    ::engine::ActorData {
                        id: self.id,
                        state: 0 as u32,
                        damage: 0,
                        resolves_collisions: false,
                        collision_filter: $filter,
                        rect: self.rect.to_sdl().unwrap(),
                        bounding_box: Some(::engine::BoundingBox::Rectangle(self.rect.clone())),
                        actor_type: $actor_type::Block,
                    }
                }
            }
        )*
    }
}
