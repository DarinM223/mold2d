use super::ActorData;
use crate::viewport::Viewport;
use sdl2::rect::Rect;

const MAX_OBJECTS: usize = 5;
const MAX_LEVELS: i32 = 10;

/// A quadtree for minimizing collision checks between actors
pub struct Quadtree<'a, Type> {
    /// The level of the current tree, (0 is root)
    level: i32,
    /// The actors that the current tree holds
    objects: Vec<ActorData<Type>>,
    /// An array of 4 subtrees to split into when parent is full
    nodes: [Option<Box<Quadtree<'a, Type>>>; 4],
    /// The bounds of the current tree
    bounds: Rect,
    /// The viewport so that all points are adjusted to the view
    viewport: &'a Viewport,
}

impl<'a, Type> Quadtree<'a, Type> {
    pub fn new(rect: Rect, viewport: &'a Viewport) -> Quadtree<'a, Type> {
        Quadtree {
            level: 0,
            objects: Vec::with_capacity(MAX_OBJECTS),
            bounds: rect,
            nodes: [None, None, None, None],
            viewport,
        }
    }

    /// Splits the node into four subnodes
    fn split(&mut self) {
        let width = (f64::from(self.bounds.width()) / 2.0) as i32;
        let height = (f64::from(self.bounds.height()) / 2.0) as i32;
        let (x, y) = (self.bounds.x(), self.bounds.y());

        if width as u32 > 0u32 && height as u32 > 0u32 {
            self.nodes[0] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new(x + width, y, width as u32, height as u32),
                nodes: [None, None, None, None],
                viewport: self.viewport,
            }));
            self.nodes[1] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new(x, y, width as u32, height as u32),
                nodes: [None, None, None, None],
                viewport: self.viewport,
            }));
            self.nodes[2] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new(x, y + height, width as u32, height as u32),
                nodes: [None, None, None, None],
                viewport: self.viewport,
            }));
            self.nodes[3] = Some(Box::new(Quadtree {
                level: self.level + 1,
                objects: Vec::with_capacity(MAX_OBJECTS),
                bounds: Rect::new(x + width, y + height, width as u32, height as u32),
                nodes: [None, None, None, None],
                viewport: self.viewport,
            }));
        }
    }

    /// Determine which node index the object belongs to
    fn index(&self, rect: &Rect) -> Option<i32> {
        self.viewport.constrain_to_viewport(rect).and_then(|rect| {
            let vert_mid = f64::from(self.bounds.x()) + f64::from(self.bounds.width()) / 2.;
            let horiz_mid = f64::from(self.bounds.y()) + f64::from(self.bounds.height()) / 2.;

            let top_quad = f64::from(rect.y()) < horiz_mid
                && f64::from(rect.y()) + f64::from(rect.height()) < horiz_mid;
            let bot_quad = f64::from(rect.y()) > horiz_mid;

            if f64::from(rect.x()) < vert_mid
                && f64::from(rect.x()) + f64::from(rect.width()) < vert_mid
            {
                if top_quad {
                    return Some(1);
                } else if bot_quad {
                    return Some(2);
                }
            } else if f64::from(rect.x()) > vert_mid {
                if top_quad {
                    return Some(0);
                } else if bot_quad {
                    return Some(3);
                }
            }

            None
        })
    }

    /// Inserts an actor into the quadtree
    pub fn insert(&mut self, actor: ActorData<Type>) {
        if self.nodes[0].is_some() {
            if let Some(index) = self.index(&actor.rect) {
                if let Some(ref mut node) = self.nodes[index as usize] {
                    node.insert(actor);
                }
                return;
            }
        }

        if self.objects.len() == MAX_OBJECTS && self.level < MAX_LEVELS {
            if self.nodes[0].is_none() {
                self.split();
            }

            let mut leftover_parent = Vec::with_capacity(MAX_OBJECTS);
            while let Some(object) = self.objects.pop() {
                if let Some(index) = self.index(&object.rect) {
                    if let Some(ref mut node) = self.nodes[index as usize] {
                        node.insert(object);
                    }
                } else {
                    leftover_parent.push(object);
                }
            }

            // Handle the overflowing actor also
            if let Some(index) = self.index(&actor.rect) {
                if let Some(ref mut node) = self.nodes[index as usize] {
                    node.insert(actor);
                }
            } else {
                leftover_parent.push(actor);
            }

            self.objects = leftover_parent;
        } else {
            self.objects.push(actor);
        }
    }

    /// Return all objects that could collide
    pub fn retrieve(&mut self, rect: &Rect) -> Vec<&ActorData<Type>> {
        let mut retrieved_values = Vec::new();
        if let Some(index) = self.index(rect) {
            if let Some(ref mut node) = self.nodes[index as usize] {
                retrieved_values.extend(node.retrieve(rect));
            }
        } else {
            // if current object is not in a quadrant add all of the children
            // since it could potentially collide with other objects in a quadrant
            for node in &mut self.nodes[..] {
                if let Some(ref mut node) = *node {
                    retrieved_values.extend(node.retrieve(rect).into_iter());
                }
            }
        }

        for object in &self.objects {
            if object.rect != *rect {
                retrieved_values.push(object);
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
