use crate::sprite::SpriteRectangle;
use crate::vector::PositionChange;
use sdl2::rect::Rect;
use std::mem;
use std::ops::{BitAnd, BitOr};

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
    (
        f64::from(rect.x()) + 0.5 * f64::from(rect.width()),
        f64::from(rect.y()) + 0.5 * f64::from(rect.height()),
    )
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollisionSide {
    Left = 0b1000,
    Right = 0b0100,
    Top = 0b0010,
    Bottom = 0b0001,
}

impl CollisionSide {
    /// Reverses a collision side
    pub fn reverse(side: CollisionSide) -> u8 {
        CollisionSide::reverse_u8(side as u8)
    }

    /// Reverses a collision side byte
    pub fn reverse_u8(side: u8) -> u8 {
        let mut side = side;

        if side & 0b0011 != 0b0011 && side & 0b0011 != 0b0000 {
            side ^= 0b0011;
        }

        if side & 0b1100 != 0b1100 && side & 0b1100 != 0b0000 {
            side ^= 0b1100;
        }

        side
    }

    pub fn print(self) {
        print_collision_side_u8(self as u8);
    }
}

impl BitAnd<CollisionSide> for CollisionSide {
    type Output = u8;

    fn bitand(self, other: CollisionSide) -> u8 {
        (self as u8) & (other as u8)
    }
}

impl BitAnd<u8> for CollisionSide {
    type Output = u8;

    fn bitand(self, other: u8) -> u8 {
        (self as u8) & other
    }
}

impl BitAnd<CollisionSide> for u8 {
    type Output = u8;

    fn bitand(self, other: CollisionSide) -> u8 {
        self & (other as u8)
    }
}

impl BitOr<CollisionSide> for CollisionSide {
    type Output = u8;

    fn bitor(self, other: CollisionSide) -> u8 {
        (self as u8) | (other as u8)
    }
}

impl BitOr<u8> for CollisionSide {
    type Output = u8;

    fn bitor(self, other: u8) -> u8 {
        (self as u8) | other
    }
}

impl BitOr<CollisionSide> for u8 {
    type Output = u8;

    fn bitor(self, other: CollisionSide) -> u8 {
        self | (other as u8)
    }
}

impl PartialEq<u8> for CollisionSide {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == *other
    }
}

impl PartialEq<CollisionSide> for u8 {
    fn eq(&self, other: &CollisionSide) -> bool {
        *self == (*other as u8)
    }
}

impl From<u8> for CollisionSide {
    fn from(side: u8) -> CollisionSide {
        assert!(
            side == CollisionSide::Left
                || side == CollisionSide::Right
                || side == CollisionSide::Bottom
                || side == CollisionSide::Top
        );
        unsafe { mem::transmute(side) }
    }
}

/// Prints the collision side as a byte
pub fn print_collision_side_u8(direction: u8) {
    print!("Collisions: (");
    let mut names = Vec::with_capacity(4);
    if direction & CollisionSide::Left != 0 {
        names.push("Left");
    }
    if direction & CollisionSide::Right != 0 {
        names.push("Right");
    }
    if direction & CollisionSide::Top != 0 {
        names.push("Top");
    }
    if direction & CollisionSide::Bottom != 0 {
        names.push("Bottom");
    }

    let names_str = names.join(",");
    print!("{}", names_str);
    print!(")");
}

/// Checks collisions for different objects
pub trait Collision<T> {
    fn collides_with(&self, other: &T) -> Option<CollisionSide>;
}

impl Collision<Rect> for Rect {
    fn collides_with(&self, other: &Rect) -> Option<CollisionSide> {
        let w = 0.5 * f64::from(self.width() + other.width());
        let h = 0.5 * f64::from(self.height() + other.height());
        let dx = center_point(self).0 - center_point(other).0;
        let dy = center_point(self).1 - center_point(other).1;

        if dx.abs() <= w && dy.abs() <= h {
            let wy = w * dy;
            let hx = h * dx;

            if wy > hx && wy > -hx {
                return Some(CollisionSide::Top);
            } else if wy > hx {
                return Some(CollisionSide::Right);
            } else if wy <= hx && wy > -hx {
                return Some(CollisionSide::Left);
            } else {
                return Some(CollisionSide::Bottom);
            }
        }

        None
    }
}

impl Collision<SpriteRectangle> for Rect {
    fn collides_with(&self, other: &SpriteRectangle) -> Option<CollisionSide> {
        self.collides_with(&other.to_sdl())
    }
}

impl Collision<Rect> for SpriteRectangle {
    fn collides_with(&self, other: &Rect) -> Option<CollisionSide> {
        self.to_sdl().collides_with(other)
    }
}

impl Collision<SpriteRectangle> for SpriteRectangle {
    fn collides_with(&self, other: &SpriteRectangle) -> Option<CollisionSide> {
        self.collides_with(&other.to_sdl())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BoundingBox {
    Rectangle(SpriteRectangle),
}

impl BoundingBox {
    pub fn apply_change(&mut self, change: &PositionChange) {
        match *self {
            BoundingBox::Rectangle(ref mut rect) => {
                rect.x += change.x;
                rect.y += change.y;
            }
        }
    }
}

impl Collision<BoundingBox> for BoundingBox {
    fn collides_with(&self, other: &BoundingBox) -> Option<CollisionSide> {
        match (self, other) {
            (BoundingBox::Rectangle(rect1), BoundingBox::Rectangle(rect2)) => {
                rect1.collides_with(rect2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::rect::Rect;

    #[test]
    fn test_collision_reverse() {
        let side = 0b1111;
        assert_eq!(CollisionSide::reverse_u8(side), 0b1111);

        let side = 0b1110;
        assert_eq!(CollisionSide::reverse_u8(side), 0b1101);

        let side = 0b0111;
        assert_eq!(CollisionSide::reverse_u8(side), 0b1011);

        let side = 0b0110;
        assert_eq!(CollisionSide::reverse_u8(side), 0b1001);

        let side = 0b0000;
        assert_eq!(CollisionSide::reverse_u8(side), 0b0000);
    }

    #[test]
    fn test_left_right_rect_collision() {
        let left_rect = Rect::new(-10, 0, 20, 20);
        let right_rect = Rect::new(0, 0, 20, 20);

        assert_eq!(
            left_rect.collides_with(&right_rect),
            Some(CollisionSide::Right)
        );
        assert_eq!(
            right_rect.collides_with(&left_rect),
            Some(CollisionSide::Left)
        );
    }

    #[test]
    fn test_up_down_rect_collision() {
        let up_rect = Rect::new(0, -20, 20, 20);
        let down_rect = Rect::new(0, 0, 20, 20);

        assert_eq!(
            up_rect.collides_with(&down_rect),
            Some(CollisionSide::Bottom)
        );
        assert_eq!(down_rect.collides_with(&up_rect), Some(CollisionSide::Top));
    }
}
