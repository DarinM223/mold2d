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

pub const COLLISION_LEFT: u8 = 0b1000;
pub const COLLISION_RIGHT: u8 = 0b0100;
pub const COLLISION_TOP: u8 = 0b0010;
pub const COLLISION_BOTTOM: u8 = 0b0001;

/// Checks collisions for different objects
pub trait Collision<T> {
    fn collides_with(&self, other: T) -> Option<u8>;
}

impl Collision<Rect> for Rect {
    fn collides_with(&self, other: Rect) -> Option<u8> {
        let w = 0.5 * (self.width() + other.width()) as f64;
        let h = 0.5 * (self.height() + other.height()) as f64;
        let dx = center_point(self).0 - center_point(&other).0;
        let dy = center_point(self).1 - center_point(&other).1;

        if dx.abs() <= w && dy.abs() <= h {
            let wy = w * dy;
            let hx = h * dx;

            if wy > hx {
                if wy > -hx {
                    return Some(COLLISION_TOP);
                } else {
                    return Some(COLLISION_LEFT);
                }
            } else {
                if wy > -hx {
                    return Some(COLLISION_RIGHT);
                } else {
                    return Some(COLLISION_BOTTOM);
                }
            }
        }

        None
    }
}

impl Collision<SpriteRectangle> for Rect {
    fn collides_with(&self, other: SpriteRectangle) -> Option<u8> {
        if let Some(rect) = other.to_sdl() {
            return self.collides_with(rect);
        }

        None
    }
}

impl Collision<Rect> for SpriteRectangle {
    fn collides_with(&self, other: Rect) -> Option<u8> {
        if let Some(rect) = self.to_sdl() {
            return rect.collides_with(other);
        }

        None
    }
}

impl Collision<SpriteRectangle> for SpriteRectangle {
    fn collides_with(&self, other: SpriteRectangle) -> Option<u8> {
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
    pub fn change_pos(&mut self, pos: &SpriteRectangle) {
        match *self {
            BoundingBox::Rectangle(ref mut rect) => {
                rect.x = pos.x;

                let sprite_h = pos.h as i32;
                let rect_h = rect.h as i32;
                rect.y = pos.y + sprite_h - rect_h;
            }
        }
    }
}

impl<'a> Collision<&'a BoundingBox> for BoundingBox {
    fn collides_with(&self, other: &'a BoundingBox) -> Option<u8> {
        match (self, other) {
            (&BoundingBox::Rectangle(ref rect1),
             &BoundingBox::Rectangle(ref rect2)) => {
                // TODO(DarinM223): avoid cloning the second rectangle
                rect1.collides_with(rect2.clone())
            }
        }
    }
}
