use std::ops::{Add, Div, Mul, Sub};

/// A two-dimensional vector
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
