use std::ops::{Add, Div, Mul, Sub};

/// A two-dimensional vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    /// Returns the length of a vector
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a normalized vector
    pub fn normalize(&self) -> Vector2D {
        Vector2D {
            x: self.x / self.len(),
            y: self.y / self.len(),
        }
    }
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

/// Represents a change of position
#[derive(Clone, Debug, PartialEq)]
pub struct PositionChange {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl PositionChange {
    pub fn new() -> PositionChange {
        PositionChange {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }

    pub fn from_vector(v: &Vector2D) -> PositionChange {
        PositionChange {
            x: v.x as i32,
            y: v.y as i32,
            w: 0,
            h: 0,
        }
    }

    pub fn left(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x - amount,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn right(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x + amount,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn up(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y - amount,
            w: self.w,
            h: self.h,
        }
    }

    pub fn down(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y + amount,
            w: self.w,
            h: self.h,
        }
    }

    pub fn shrink_height_top(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y + amount,
            w: self.w,
            h: self.h - amount,
        }
    }

    pub fn grow_height_top(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y - amount,
            w: self.w,
            h: self.h + amount,
        }
    }

    pub fn shrink_height_bot(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h - amount,
        }
    }

    pub fn grow_height_bot(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h + amount,
        }
    }

    pub fn shrink_width_left(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x - amount,
            y: self.y,
            w: self.w - amount,
            h: self.h,
        }
    }

    pub fn grow_width_left(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x + amount,
            y: self.y,
            w: self.w + amount,
            h: self.h,
        }
    }

    pub fn shrink_width_right(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y,
            w: self.w - amount,
            h: self.h,
        }
    }

    pub fn grow_width_right(&self, amount: i32) -> PositionChange {
        PositionChange {
            x: self.x,
            y: self.y,
            w: self.w + amount,
            h: self.h,
        }
    }

    pub fn chain(&self, change: &PositionChange) -> PositionChange {
        PositionChange {
            x: self.x + change.x,
            y: self.y + change.y,
            w: self.w + change.w,
            h: self.h + change.h,
        }
    }

    pub fn to_vector(&self) -> Vector2D {
        Vector2D {
            x: f64::from(self.x),
            y: f64::from(self.y),
        }
    }
}
