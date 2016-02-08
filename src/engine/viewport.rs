use context::Window;
use level::GRID_SIZE;
use sdl2::rect::{Point, Rect};

pub struct Viewport {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Viewport {
    pub fn new(window: &Window, center: (i32, i32)) -> Viewport {
        Viewport {
            x: center.0,
            y: center.1,
            width: window.width,
            height: window.height,
        }
    }

    pub fn update(&mut self, change: (i32, i32)) {
        self.x += change.0;
        self.y += change.1;
    }

    pub fn set_position(&mut self, point: (i32, i32)) {
        self.x = point.0;
        self.y = point.1;
    }

    pub fn in_viewport(&self, point: Point) -> bool {
        let x_min = (self.x as f64) - (self.width as f64) / 2.0;
        let x_max = (self.x as f64) + (self.width as f64) / 2.0;
        let y_min = (self.y as f64) - (self.height as f64) / 2.0;
        let y_max = (self.y as f64) + (self.height as f64) / 2.0;

        (point.x() as f64) >= x_min && (point.x() as f64) <= x_max && (point.y() as f64) >= y_min && (point.y() as f64) <= y_max
    }

    /// Returns the point in the game relative to the viewpoint
    pub fn relative_point(&self, map_point: (i32, i32)) -> (i32, i32) {
        let left_margin = 100;
        let top_margin = self.height as i32 - 100 - GRID_SIZE * 4;

        (map_point.0 - self.x + left_margin,
         map_point.1 - self.y + top_margin)
    }

    /// Returns a rectangle in viewport coordinates or None if not in viewport
    pub fn constrain_to_viewport(&self, rect: &Rect) -> Option<Rect> {
        if self.in_viewport(Point::new(rect.x(), rect.y())) {
            let (x, y) = self.relative_point((rect.x(), rect.y()));
            Some(Rect::new_unwrap(x, y, rect.width(), rect.height()))
        } else {
            None
        }
    }
}
