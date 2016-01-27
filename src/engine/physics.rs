use engine::sprite::SpriteRectangle;

pub struct PositionUpdater {
    vel: (f64, f64),
    forces: Vec<(i32, i32)>,
}

impl PositionUpdater {
    pub fn new() -> PositionUpdater {
        PositionUpdater {
            vel: (0.0, 0.0),
            forces: Vec::new(),
        }
    }

    pub fn add_force(&mut self, vec: (i32, i32)) {
        self.forces.push(vec);
    }

    pub fn update(&mut self, rect: &mut SpriteRectangle, elapsed: f64) {
        let (mut fx, mut fy) = (0, 0);
        for force in &self.forces {
            fx += force.0;
            fy += force.1;
        }

        self.vel.0 += fx as f64 * elapsed;
        self.vel.1 += fy as f64 * elapsed;

        rect.x += self.vel.0 as i32;
        rect.y += self.vel.1 as i32;
    }
}
