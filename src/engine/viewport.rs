use collision::center_point;
use context::Window;
use level::GRID_SIZE;
use sdl2::rect::{Point, Rect};

/// Constrains coordinates from an open world into the current window view
/// This allows for scrolling for levels larger than the current screen
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    /// Width and height of the window
    pub window_dimensions: (i32, i32),
    /// Width and height of the map
    pub map_dimensions: (i32, i32),
}

fn calc_viewport_point(cc: f64, vc: f64, mc: f64) -> f64 {
    let half = vc / 2.0;

    ((cc - half).max(0.0)).min((mc - vc).min((cc - half).abs()))
}

impl Viewport {
    pub fn new(window: &Window, map_dimensions: (i32, i32)) -> Viewport {
        Viewport {
            x: 0,
            y: 0,
            window_dimensions: (window.width as i32, window.height as i32),
            map_dimensions: map_dimensions,
        }
    }

    pub fn set_position(&mut self, new_center: (i32, i32)) {
        let new_x = calc_viewport_point(new_center.0 as f64,
                                        self.window_dimensions.0 as f64,
                                        self.map_dimensions.0 as f64);
        let new_y = calc_viewport_point(new_center.1 as f64,
                                        self.window_dimensions.1 as f64,
                                        self.map_dimensions.1 as f64);

        self.x = new_x as i32;
        self.y = new_y as i32;
    }

    /// Returns true if the point is inside the viewport, false otherwise
    pub fn in_viewport(&self, point: (i32, i32)) -> bool {
        let margin = 32;

        let (v_min_x, v_max_x) = (self.x - margin, self.x + self.window_dimensions.0);
        let (v_min_y, v_max_y) = (self.y - margin, self.y + self.window_dimensions.1);

        point.0 >= v_min_x && point.0 <= v_max_x && point.1 >= v_min_y && point.1 <= v_max_y
    }

    /// Returns the point in the game relative to the viewpoint
    pub fn relative_point(&self, map_point: (i32, i32)) -> (i32, i32) {
        (map_point.0 - self.x, map_point.1 - self.y)
    }

    /// Returns a rectangle in viewport coordinates or None if not in viewport
    pub fn constrain_to_viewport(&self, rect: &Rect) -> Option<Rect> {
        let center = center_point(rect);
        if self.in_viewport((center.0 as i32, center.1 as i32)) {
            let (x, y) = self.relative_point((center.0 as i32, center.1 as i32));
            Some(Rect::new_unwrap(x, y, rect.width(), rect.height()))
        } else {
            None
        }
    }
}
