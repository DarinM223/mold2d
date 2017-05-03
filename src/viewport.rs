use collision::center_point;
use context::Window;
use sdl2::rect::Rect;

/// Calculates the origin coordinate for the viewport
/// given the center coordinate, the canvas coordinate, and the map coordinate
fn calc_viewport_point(center_coord: f64, window_coord: f64, map_coord: f64) -> f64 {
    let half = window_coord / 2.0;

    ((center_coord - half).max(0.0)).min((map_coord - window_coord).min((center_coord - half)
                                                                            .abs()))
}

/// Constrains coordinates from an open world into the current window view
/// This allows for scrolling for levels larger than the current screen
#[derive(Clone)]
pub struct Viewport {
    /// The x value of the center coordinate of the viewport
    pub x: i32,
    /// The y value of the center coordinate of the viewport
    pub y: i32,
    /// Width and height of the window
    pub window_dimensions: (i32, i32),
    /// Width and height of the map
    pub map_dimensions: (i32, i32),
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

    /// Returns true if the rectangle is inside the viewport, false otherwise
    pub fn rect_in_viewport(&self, rect: &Rect) -> bool {
        let x_plus_width = rect.x() + rect.width() as i32;
        let y_plus_height = rect.y() + rect.height() as i32;
        let rect_points = [(rect.x(), rect.y()),
                           (x_plus_width, rect.y()),
                           (rect.x(), y_plus_height),
                           (x_plus_width, y_plus_height)];

        for point in rect_points.iter() {
            if self.in_viewport(*point) {
                return true;
            }
        }

        false
    }

    /// Returns a rectangle in viewport coordinates or None if not in viewport
    pub fn constrain_to_viewport(&self, rect: &Rect) -> Option<Rect> {
        if self.rect_in_viewport(rect) {
            let center = center_point(rect);
            let (x, y) = self.relative_point((center.0 as i32, center.1 as i32));
            Some(Rect::new(x, y, rect.width(), rect.height()))
        } else {
            None
        }
    }
}
