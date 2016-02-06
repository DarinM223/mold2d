use engine::sprite::SpriteRectangle;
use sdl2::rect::{Point, Rect};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f64> for Vector2D {
    type Output = Vector2D;

    fn add(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Add<Vector2D> for f64 {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self + other.x,
            y: self + other.y,
        }
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f64> for Vector2D {
    type Output = Vector2D;

    fn sub(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Sub<Vector2D> for f64 {
    type Output = Vector2D;

    fn sub(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self - other.x,
            y: self - other.y,
        }
    }
}

impl Mul<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn mul(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Div<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn div(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, other: f64) -> Vector2D {
        Vector2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Div<Vector2D> for f64 {
    type Output = Vector2D;

    fn div(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self / other.x,
            y: self / other.y,
        }
    }
}

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

pub trait Collision<T> {
    fn collides_with(&self, other: T) -> Option<CollisionSide>;
}

impl Collision<Rect> for Rect {
    fn collides_with(&self, other: Rect) -> Option<CollisionSide> {
        let w = 0.5 * (self.width() + other.width()) as f64;
        let h = 0.5 * (self.height() + other.height()) as f64;
        let dx = (self.x() - other.x()) as f64;
        let dy = (self.y() - other.y()) as f64;

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
