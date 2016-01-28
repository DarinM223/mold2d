use engine::geo_utils::GeoUtils;
use engine::sprite::SpriteRectangle;
use engine::view::Actor;
use sdl2::rect::Rect;
use std::collections::HashMap;

pub struct PositionUpdater {
    vel: (f64, f64),
    forces: HashMap<String, (i32, i32)>,
}

impl PositionUpdater {
    pub fn new() -> PositionUpdater {
        PositionUpdater {
            vel: (0.0, 0.0),
            forces: HashMap::new(),
        }
    }

    pub fn add_force(&mut self, force: &str, vec: (i32, i32)) {
        if self.forces.contains_key(force) {
            let mut force = self.forces.get_mut(force).unwrap();
            force.0 += vec.0;
            force.1 += vec.1;
        } else {
            self.forces.insert(force.to_owned(), vec);
        }
    }

    pub fn remove_force(&mut self, force: &str) {
        self.forces.remove(force);
    }

    pub fn update(&mut self, rect: &mut SpriteRectangle, actors: Vec<Rect>, elapsed: f64) {
        let (mut fx, mut fy) = (0, 0);
        for (_, force) in &self.forces {
            fx += force.0;
            fy += force.1;
        }

        self.vel.0 += fx as f64 * elapsed;
        self.vel.1 += fy as f64 * elapsed;

        let new_x = rect.x + self.vel.0 as i32;
        let new_y = rect.y + self.vel.1 as i32;

        let new_rect = Rect::new_unwrap(new_x, new_y, rect.w, rect.h);

        let mut collision = false;
        for rect in actors {
            if GeoUtils::rect_overlaps_rect(new_rect, rect) {
                collision = true;
                break;
            }
        }

        if !collision {
            rect.x += self.vel.0 as i32;
            rect.y += self.vel.1 as i32;
        }
    }
}
