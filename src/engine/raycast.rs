use vector::Vector2D;

/// Returns the point where two lines intersect
/// if there is an intersection or None otherwise.
/// p0 and p1 are the points of the first line and
/// p2 and p3 are the points of the second line.
/// http://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/1968345#1968345
pub fn get_intersection(p0: (f64, f64),
                        p1: (f64, f64),
                        p2: (f64, f64),
                        p3: (f64, f64))
                        -> Option<(f64, f64)> {

    let s1 = (p1.0 - p0.0, p1.1 - p0.1);
    let s2 = (p3.0 - p2.0, p3.1 - p2.1);

    let s = (-s1.1 * (p0.0 - p2.0) + s1.0 * (p0.1 - p2.1)) / (-s2.0 * s1.1 + s1.0 * s2.1);
    let t = (s2.0 * (p0.1 - p2.1) - s2.1 * (p0.0 - p2.0)) / (-s2.0 * s1.1 + s1.0 * s2.1);

    if s >= 0. && s <= 1. && t >= 0. && t <= 1. {
        // Collision detected
        let x = p0.0 + (t * s1.0);
        let y = p0.1 + (t * s1.1);
        return Some((x, y));
    } else {
        None
    }
}

#[derive(Clone)]
pub struct Segment {
    /// The point that the segment starts out at
    point: (f64, f64),
    /// The magnitude and the direction of the segment
    vector: Vector2D,
}

impl Segment {
    /// Checks intersection for two segments.
    /// If the two segments intersect it returns the distance of the end point
    /// to the collided segment, otherwise it returns None
    pub fn intersects(&self, other: &Segment) -> Option<(f64, f64)> {
        let p0 = self.point;
        let p1 = (self.point.0 + self.vector.x, self.point.1 + self.vector.y);
        let p2 = other.point;
        let p3 = (other.point.0 + other.vector.x,
                  other.point.1 + other.vector.y);

        get_intersection(p0, p1, p2, p3)
    }

    /// Returns the normalized segment
    pub fn normalize(&self) -> Segment {
        Segment {
            point: self.point,
            vector: self.vector.normalize(),
        }
    }

    /// Returns a segment with a magnitude shortened by a certain amount
    pub fn shorten(&self, amount: f64) -> Segment {
        let old_length = self.len();
        let norm_seg = self.normalize();
        let magnitude = old_length - amount;

        Segment {
            point: self.point,
            vector: norm_seg.vector * magnitude,
        }
    }

    /// Returns the length of the segment
    pub fn len(&self) -> f64 {
        self.vector.len()
    }
}

pub trait Polygon {
    /// Returns the sides in a polygon shape as a dynamic array of segments
    fn sides(&self) -> Vec<Segment>;
}

/// Shortens a ray segment agains a polygon
pub fn shorten_ray<P: Polygon>(ray: &Segment, poly: &P) -> Segment {
    poly.sides().iter().fold(ray.clone(), |ray, side| {
        if let Some((int_x, int_y)) = ray.intersects(side) {
            // Shorten the ray by the distance between the intersection point
            // and the endpoint of the ray
            let (end_x, end_y) = (ray.point.0 + ray.vector.x, ray.point.1 + ray.vector.y);
            let distance = ((end_x - int_x).powi(2) + (end_y - int_y).powi(2)).sqrt();
            ray.shorten(distance)
        } else {
            ray
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector::Vector2D;

    fn assert_float(a: f64, b: f64) {
        assert!((a - b).abs() < 0.000000001);
    }

    #[test]
    fn test_normalize() {
        let segment = Segment {
            point: (1., 1.),
            vector: Vector2D { x: 11., y: 11. },
        };

        let normalized_segment = segment.normalize();
        assert_float(normalized_segment.len(), 1.);
    }

    #[test]
    fn test_shorten() {
        let segment = Segment {
            point: (1., 1.),
            vector: Vector2D { x: 3., y: 3. },
        };
        let shortened_segment = segment.shorten(2.);

        assert_eq!(segment.point, shortened_segment.point);
        assert_float(shortened_segment.len(), segment.len() - 2.);
    }
}
