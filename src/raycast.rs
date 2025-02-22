use crate::collision::CollisionSide;
use crate::vector::Vector2D;
use crate::viewport::Viewport;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Renderer;
use std::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    /// The point that the segment starts out at
    pub point: (f64, f64),
    /// The magnitude and the direction of the segment
    pub vector: Vector2D,
}

impl Segment {
    /// Checks intersection for two segments.
    /// If the two segments intersect it returns the intersection point,
    /// otherwise it returns None
    pub fn intersects(&self, other: &Segment) -> Option<(f64, f64)> {
        let p0 = self.point;
        let p1 = (self.point.0 + self.vector.x, self.point.1 + self.vector.y);
        let p2 = other.point;
        let p3 = (
            other.point.0 + other.vector.x,
            other.point.1 + other.vector.y,
        );

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
        let old_length = self.length();
        let norm_seg = self.normalize();
        let magnitude = old_length - amount;

        Segment {
            point: self.point,
            vector: norm_seg.vector * magnitude,
        }
    }

    /// Returns the length of the segment
    pub fn length(&self) -> f64 {
        self.vector.length()
    }

    pub fn render(
        &self,
        color: Color,
        viewport: &mut Viewport,
        renderer: &mut Renderer,
    ) -> Result<(), Box<dyn Error>> {
        let (rx, ry) = viewport.relative_point((self.point.0 as i32, self.point.1 as i32));
        let p1 = Point::new(rx, ry);
        let p2 = Point::new(rx + (self.vector.x as i32), ry + (self.vector.y as i32));
        renderer.set_draw_color(color);
        renderer.draw_line(p1, p2).map_err(From::from)
    }
}

pub trait Polygon {
    /// Returns the sides in a polygon shape as a dynamic array of segments
    fn sides(&self) -> Vec<Segment>;

    /// Given a side number of the polygon, returns the side of the collision
    fn collision_from_side(&self, id: usize) -> Option<CollisionSide>;
}

impl Polygon for Rect {
    fn sides(&self) -> Vec<Segment> {
        let (f_x, f_y) = (f64::from(self.x()), f64::from(self.y()));
        let (f_w, f_h) = (f64::from(self.width()), f64::from(self.height()));

        vec![
            Segment {
                point: (f_x, f_y),
                vector: Vector2D { x: 0., y: f_h },
            },
            Segment {
                point: (f_x, f_y + f_h),
                vector: Vector2D { x: f_w, y: 0. },
            },
            Segment {
                point: (f_x + f_w, f_y + f_h),
                vector: Vector2D { x: 0., y: -f_h },
            },
            Segment {
                point: (f_x + f_w, f_y),
                vector: Vector2D { x: -f_w, y: 0. },
            },
        ]
    }

    fn collision_from_side(&self, id: usize) -> Option<CollisionSide> {
        match id {
            0 => Some(CollisionSide::Left),
            1 => Some(CollisionSide::Bottom),
            2 => Some(CollisionSide::Right),
            3 => Some(CollisionSide::Top),
            _ => None,
        }
    }
}

/// Shortens a ray segment agains a polygon
pub fn shorten_ray<P: Polygon>(ray: &mut Segment, poly: &P) -> Option<CollisionSide> {
    for (id, side) in poly.sides().iter().enumerate() {
        if let Some((int_x, int_y)) = ray.intersects(side) {
            // Shorten the ray by the distance between the intersection point
            // and the endpoint of the ray
            let (end_x, end_y) = (ray.point.0 + ray.vector.x, ray.point.1 + ray.vector.y);
            let distance = ((end_x - int_x).powi(2) + (end_y - int_y).powi(2)).sqrt();
            let coll_side = poly.collision_from_side(id);
            *ray = ray.shorten(distance);
            return coll_side;
        }
    }
    None
}

/// Returns the point where two lines intersect
/// if there is an intersection or None otherwise.
/// p0 and p1 are the points of the first line and
/// p2 and p3 are the points of the second line.
/// http://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/1968345#1968345
fn get_intersection(
    p0: (f64, f64),
    p1: (f64, f64),
    p2: (f64, f64),
    p3: (f64, f64),
) -> Option<(f64, f64)> {
    let s1 = (p1.0 - p0.0, p1.1 - p0.1);
    let s2 = (p3.0 - p2.0, p3.1 - p2.1);

    let s = (-s1.1 * (p0.0 - p2.0) + s1.0 * (p0.1 - p2.1)) / (-s2.0 * s1.1 + s1.0 * s2.1);
    let t = (s2.0 * (p0.1 - p2.1) - s2.1 * (p0.0 - p2.0)) / (-s2.0 * s1.1 + s1.0 * s2.1);

    if (0. ..=1.).contains(&s) && (0. ..=1.).contains(&t) {
        // Collision detected
        let x = p0.0 + (t * s1.0);
        let y = p0.1 + (t * s1.1);
        Some((x, y))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collision::CollisionSide;
    use crate::vector::Vector2D;
    use sdl2::rect::Rect;

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
        assert_float(normalized_segment.length(), 1.);
    }

    #[test]
    fn test_rect_sides() {
        let rect = Rect::new(0, 0, 20, 20);
        let sides = rect.sides();

        assert_eq!(
            sides,
            vec![
                Segment {
                    point: (0., 0.),
                    vector: Vector2D { x: 0., y: 20. },
                },
                Segment {
                    point: (0., 20.),
                    vector: Vector2D { x: 20., y: 0. },
                },
                Segment {
                    point: (20., 20.),
                    vector: Vector2D { x: 0., y: -20. },
                },
                Segment {
                    point: (20., 0.),
                    vector: Vector2D { x: -20., y: 0. },
                }
            ]
        );
    }

    #[test]
    fn test_shorten() {
        let segment = Segment {
            point: (1., 1.),
            vector: Vector2D { x: 3., y: 3. },
        };
        let shortened_segment = segment.shorten(2.);

        assert_eq!(segment.point, shortened_segment.point);
        assert_float(shortened_segment.length(), segment.length() - 2.);
    }

    #[test]
    fn test_intersect() {
        let segment1 = Segment {
            point: (1., 1.),
            vector: Vector2D { x: 2., y: 2. },
        };
        let segment2 = Segment {
            point: (0., 2.),
            vector: Vector2D { x: 3., y: 0. },
        };

        let intersect_point = segment1.intersects(&segment2);
        assert_eq!(intersect_point, Some((2., 2.)));
    }

    #[test]
    fn test_shorten_ray_left() {
        let rect = Rect::new(2, 3, 2, 2);
        let mut segment = Segment {
            point: (0., 3.),
            vector: Vector2D { x: 4., y: 0. },
        };

        let side = shorten_ray(&mut segment, &rect);
        assert_eq!(side, Some(CollisionSide::Left));
        assert_eq!(
            segment,
            Segment {
                point: (0., 3.),
                vector: Vector2D { x: 2., y: 0. },
            }
        );

        let mut segment = Segment {
            point: (0., 2.),
            vector: Vector2D { x: 4., y: 0. },
        };

        let _side = shorten_ray(&mut segment, &rect);
        assert_eq!(
            segment,
            Segment {
                point: (0., 2.),
                vector: Vector2D { x: 4., y: 0. },
            }
        );
    }

    #[test]
    fn test_shorten_ray_top() {
        let rect = Rect::new(2, 3, 2, 2);
        let mut segment = Segment {
            point: (3., 0.),
            vector: Vector2D { x: 0., y: 4. },
        };

        let side = shorten_ray(&mut segment, &rect);
        assert_eq!(side, Some(CollisionSide::Top));
        assert_eq!(
            segment,
            Segment {
                point: (3., 0.),
                vector: Vector2D { x: 0., y: 3. },
            }
        );
    }
}
