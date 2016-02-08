use sdl2::rect::Rect;
use view::ActorData;
use viewport::Viewport;

const MAX_OBJECTS: usize = 10;
const MAX_LEVELS: i32 = 5;

pub struct Quadtree<'a> {
    level: i32,
    objects: Vec<ActorData>,
    nodes: [Option<Box<Quadtree<'a>>>; 4],
    bounds: Rect,
    viewport: &'a Viewport,
}

impl<'a> Quadtree<'a> {
    pub fn new(rect: Rect, viewport: &'a Viewport) -> Quadtree<'a> {
        Quadtree {
            level: 0,
            objects: Vec::new(),
            bounds: rect,
            nodes: [None, None, None, None],
            viewport: viewport,
        }
    }

    /// Splits the node into four subnodes
    fn split(&mut self) {
        let width = self.bounds.width() as i32 / 2;
        let height = self.bounds.height() as i32 / 2;
        let (x, y) = (self.bounds.x(), self.bounds.y());

        self.nodes[0] = Some(Box::new(Quadtree {
            level: self.level + 1,
            objects: Vec::new(),
            bounds: Rect::new_unwrap(x + width, y, width as u32, height as u32),
            nodes: [None, None, None, None],
            viewport: self.viewport,
        }));
        self.nodes[1] = Some(Box::new(Quadtree {
            level: self.level + 1,
            objects: Vec::new(),
            bounds: Rect::new_unwrap(x, y, width as u32, height as u32),
            nodes: [None, None, None, None],
            viewport: self.viewport,
        }));
        self.nodes[2] = Some(Box::new(Quadtree {
            level: self.level + 1,
            objects: Vec::new(),
            bounds: Rect::new_unwrap(x, y + height, width as u32, height as u32),
            nodes: [None, None, None, None],
            viewport: self.viewport,
        }));
        self.nodes[3] = Some(Box::new(Quadtree {
            level: self.level + 1,
            objects: Vec::new(),
            bounds: Rect::new_unwrap(x + width, y + height, width as u32, height as u32),
            nodes: [None, None, None, None],
            viewport: self.viewport,
        }));
    }

    /// Determine which node index the object belongs to
    fn index(&self, rect: &Rect) -> Option<i32> {
        if let Some(rect) = self.viewport.constrain_to_viewport(rect) {
            let vert_mid = (self.bounds.x() as f64) + (self.bounds.width() as f64) / 2.;
            let horiz_mid = (self.bounds.y() as f64) + (self.bounds.height() as f64) / 2.;

            let top_quad = (rect.y() as f64) < horiz_mid &&
                           (rect.y() as f64) + (rect.height() as f64) < horiz_mid;
            let bot_quad = (rect.y() as f64) > horiz_mid;

            if (rect.x() as f64) < vert_mid &&
               (rect.x() as f64) + (rect.width() as f64) < vert_mid {
                if top_quad {
                    return Some(1);
                } else if bot_quad {
                    return Some(2);
                }
            } else if (rect.x() as f64) > vert_mid {
                if top_quad {
                    return Some(0);
                } else if bot_quad {
                    return Some(3);
                }
            }
        }

        None
    }

    /// Inserts an actor into the quadtree
    pub fn insert(&mut self, actor: ActorData) {
        if let Some(_) = self.nodes[0] {
            if let Some(index) = self.index(&actor.rect) {
                if let Some(ref mut node) = self.nodes[index as usize] {
                    node.insert(actor);
                }
                return;
            }
        }

        self.objects.push(actor);

        if self.objects.len() > MAX_OBJECTS && self.level < MAX_LEVELS {
            if let None = self.nodes[0] {
                self.split();
            }

            while !self.objects.is_empty() {
                let object = self.objects.pop().unwrap();
                if let Some(index) = self.index(&object.rect) {
                    if let Some(ref mut node) = self.nodes[index as usize] {
                        node.insert(object);
                    }
                }
            }
        }
    }

    /// Return all objects that could collide
    pub fn retrieve(&mut self, rect: &Rect) -> Vec<&ActorData> {
        let mut retrieved_values = Vec::new();
        if let Some(index) = self.index(rect) {
            if let Some(ref mut node) = self.nodes[index as usize] {
                retrieved_values.extend(node.retrieve(rect).into_iter());
            }
        }

        for object in &self.objects {
            retrieved_values.push(object);
        }

        retrieved_values
    }
}
