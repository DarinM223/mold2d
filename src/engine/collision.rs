use sdl2::rect::Rect;
use sprite::SpriteRectangle;

/// Checks if a rectangle contains another rectangle
pub fn rect_contains_rect(parent: Rect, child: Rect) -> bool {
    let x_min = child.x();
    let x_max = x_min + child.width() as i32;
    let y_min = child.y();
    let y_max = y_min + child.height() as i32;

    let check_xmin = x_min >= parent.x() && x_min <= parent.x() + parent.width() as i32;
    let check_xmax = x_max >= parent.x() && x_max <= parent.x() + parent.width() as i32;
    let check_ymin = y_min >= parent.y() && y_min <= parent.y() + parent.height() as i32;
    let check_ymax = y_max >= parent.y() && y_max <= parent.y() + parent.height() as i32;

    check_xmin && check_xmax && check_ymin && check_ymax
}

/// Returns the center point of a rectangle as a tuple of decimals
pub fn center_point(rect: &Rect) -> (f64, f64) {
    ((rect.x() as f64) + 0.5 * (rect.width() as f64),
     (rect.y() as f64) + 0.5 * (rect.height() as f64))
}

/// The side of the actor being collided into
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

/// Checks collisions for different objects
pub trait Collision<T> {
    fn collides_with(&self, other: T) -> Option<CollisionSide>;
}

impl Collision<Rect> for Rect {
    fn collides_with(&self, other: Rect) -> Option<CollisionSide> {
        let w = 0.5 * (self.width() + other.width()) as f64;
        let h = 0.5 * (self.height() + other.height()) as f64;
        let dx = center_point(self).0 - center_point(&other).0;
        let dy = center_point(self).1 - center_point(&other).1;

        if dx.abs() <= w && dy.abs() <= h {
            let wy = w * dy;
            let hx = h * dx;

            if wy > hx {
                if wy > -hx {
                    return Some(CollisionSide::Top);
                } else {
                    return Some(CollisionSide::Left);
                }
            } else {
                if wy > -hx {
                    return Some(CollisionSide::Right);
                } else {
                    return Some(CollisionSide::Bottom);
                }
            }
        }

        None
    }
}

impl Collision<SpriteRectangle> for Rect {
    fn collides_with(&self, other: SpriteRectangle) -> Option<CollisionSide> {
        if let Some(rect) = other.to_sdl() {
            return self.collides_with(rect);
        }

        None
    }
}

impl Collision<Rect> for SpriteRectangle {
    fn collides_with(&self, other: Rect) -> Option<CollisionSide> {
        if let Some(rect) = self.to_sdl() {
            return rect.collides_with(other);
        }

        None
    }
}

impl Collision<SpriteRectangle> for SpriteRectangle {
    fn collides_with(&self, other: SpriteRectangle) -> Option<CollisionSide> {
        if let Some(rect) = other.to_sdl() {
            return self.collides_with(rect);
        }

        None
    }
}

#[derive(Clone, PartialEq)]
pub enum BoundingBox {
    Rectangle(SpriteRectangle),
}

impl BoundingBox {
    pub fn change_pos(&mut self, point: (i32, i32), dimensions: (u32, u32)) {
        match *self {
            BoundingBox::Rectangle(ref mut rect) => {
                rect.x = point.0;

                let sprite_h = dimensions.1 as i32;
                let rect_h = rect.h as i32;
                rect.y = point.1 + sprite_h - rect_h;
            }
        }
    }
}

impl Collision<BoundingBox> for BoundingBox {
    fn collides_with(&self, other: BoundingBox) -> Option<CollisionSide> {
        match (self, other) {
            (&BoundingBox::Rectangle(ref rect1),
             BoundingBox::Rectangle(ref rect2)) => {
                // TODO(DarinM223): avoid cloning the second rectangle
                rect1.collides_with(rect2.clone())
            }
        }
    }
}
