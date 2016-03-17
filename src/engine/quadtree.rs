use actor_manager::ActorManager;
use sdl2::rect::Rect;
use view::ActorData;

const MAX_OBJECTS: usize = 50;
const MAX_LEVELS: i32 = 100;

/// A quadtree for minimizing collision checks between actors
pub struct Quadtree {
    /// The level of the current tree, (0 is root)
    level: i32,
    /// The actors that the current tree holds
    objects: Vec<i32>,
    /// An array of 4 subtrees to split into when parent is full
    nodes: [Option<Box<Quadtree>>; 4],
    /// The bounds of the current tree
    bounds: Rect,
}

impl Quadtree {
    pub fn new(rect: Rect) -> Quadtree {
        Quadtree {
            level: 0,
            objects: Vec::with_capacity(MAX_OBJECTS),
            bounds: rect,
            nodes: [None, None, None, None],
        }
    }

    /// Splits the node into four subnodes
    fn split(&mut self) {
        let width = ((self.bounds.width() as f64) / 2.0) as i32;
        let height = ((self.bounds.height() as f64) / 2.0) as i32;
        let (x, y) = (self.bounds.x(), self.bounds.y());

        if width as u32 > 0u32 && height as u32 > 0u32 {
            self.nodes[0] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new_unwrap(x + width, y, width as u32, height as u32),
                nodes: [None, None, None, None],
            }));
            self.nodes[1] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new_unwrap(x, y, width as u32, height as u32),
                nodes: [None, None, None, None],
            }));
            self.nodes[2] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new_unwrap(x, y + height, width as u32, height as u32),
                nodes: [None, None, None, None],
            }));
            self.nodes[3] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new_unwrap(x + width, y + height, width as u32, height as u32),
                nodes: [None, None, None, None],
            }));
        }
    }

    /// Determine which node index the object belongs to
    fn index_rect(&self, rect: &Rect) -> Option<i32> {
        let vert_mid = (self.bounds.x() as f64) + (self.bounds.width() as f64) / 2.;
        let horiz_mid = (self.bounds.y() as f64) + (self.bounds.height() as f64) / 2.;

        let top_quad = (rect.y() as f64) < horiz_mid &&
                       (rect.y() as f64) + (rect.height() as f64) < horiz_mid;
        let bot_quad = (rect.y() as f64) > horiz_mid;

        if (rect.x() as f64) < vert_mid && (rect.x() as f64) + (rect.width() as f64) < vert_mid {
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

        None
    }

    /// Determine which node index the actor belongs to
    fn index<Type, Message>(&self,
                            actor: i32,
                            actor_manager: &mut ActorManager<Type, Message>)
                            -> Option<i32> {
        if let Some(ref rect) = actor_manager.get_mut(actor).map(|actor| actor.data().rect) {
            return self.index_rect(rect);
        }

        None
    }

    /// Inserts an actor into the quadtree
    pub fn insert<Type, Message>(&mut self,
                                 actor: i32,
                                 actor_manager: &mut ActorManager<Type, Message>) {
        if let Some(_) = self.nodes[0] {
            if let Some(index) = self.index(actor, actor_manager) {
                if let Some(ref mut node) = self.nodes[index as usize] {
                    node.insert(actor, actor_manager);
                }
                return;
            }
        }

        if self.objects.len() == MAX_OBJECTS && self.level < MAX_LEVELS {
            if let None = self.nodes[0] {
                self.split();
            }

            let mut leftover_parent = Vec::with_capacity(MAX_OBJECTS);
            while !self.objects.is_empty() {
                let object = self.objects.pop().unwrap();
                if let Some(index) = self.index(object, actor_manager) {
                    if let Some(ref mut node) = self.nodes[index as usize] {
                        node.insert(object, actor_manager);
                    }
                } else {
                    leftover_parent.push(object);
                }
            }

            // Handle the overflowing actor also
            if let Some(index) = self.index(actor, actor_manager) {
                if let Some(ref mut node) = self.nodes[index as usize] {
                    node.insert(actor, actor_manager);
                }
            } else {
                leftover_parent.push(actor);
            }

            self.objects = leftover_parent;
        } else {
            self.objects.push(actor);
        }
    }

    /// Return all object indexes that could collide
    pub fn retrieve(&mut self, id: i32, rect: &Rect) -> Vec<i32> {
        let mut retrieved_values = Vec::new();
        if let Some(index) = self.index_rect(rect) {
            if let Some(ref mut node) = self.nodes[index as usize] {
                retrieved_values.extend(node.retrieve(id, rect).into_iter());
            }
        } else {
            // if current object is not in a quadrant add all of the children
            // since it could potentially collide with other objects in a quadrant
            for node in &mut self.nodes[..] {
                if let Some(ref mut node) = *node {
                    retrieved_values.extend(node.retrieve(id, rect).into_iter());
                }
            }
        }

        for object in &self.objects {
            if *object != id {
                retrieved_values.push(*object);
            }
        }

        retrieved_values
    }

    /// Returns the total number of elements in the quadtree
    pub fn len(&self) -> usize {
        let mut l = self.objects.len();

        for i in 0..4 {
            if let Some(ref node) = self.nodes[i] {
                l += node.len();
            }
        }

        l
    }
}
