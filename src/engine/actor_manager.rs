use context::Window;
use quadtree::Quadtree;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use std::collections::HashMap;
use view::Actor;
use viewport::Viewport;

pub trait ActorFromToken<Type, Message> {
    /// Returns an actor given a token
    fn actor_from_token(&self,
                        token: char,
                        id: i32,
                        position: (i32, i32),
                        renderer: &mut Renderer)
                        -> Box<Actor<Type, Message>>;
}

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager<Type, Message> {
    next_id: i32,
    pub actors: HashMap<i32, Box<Actor<Type, Message>>>,
    temporary: Option<i32>,
    actor_gen: Box<ActorFromToken<Type, Message>>,
    quadtree: Option<Quadtree>,
}

impl<Type, Message> ActorManager<Type, Message> {
    pub fn new(actor_gen: Box<ActorFromToken<Type, Message>>,
               window: &Window)
               -> ActorManager<Type, Message> {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
            temporary: None,
            actor_gen: actor_gen,
            quadtree: Some(Quadtree::new(Rect::new_unwrap(0, 0, window.width, window.height))),
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, token: char, position: (i32, i32), renderer: &mut Renderer) {
        let actor = self.actor_gen.actor_from_token(token, self.next_id, position, renderer);
        self.actors.insert(self.next_id, actor);
        if let Some(mut quadtree) = self.quadtree.take() {
            quadtree.insert(self.next_id, self);
            self.quadtree = Some(quadtree);
        }
        self.next_id += 1;
    }

    /// Remove an actor from the actors
    pub fn remove(&mut self, id: i32) {
        if let Some(temp_id) = self.temporary {
            if id == temp_id {
                self.temporary = None;
            }
        }

        // TODO(DarinM223): remove from quadtree as well
        self.actors.remove(&id);
    }

    /// Retrieves actors close to the given actor
    pub fn retrieve_actors(&mut self, id: i32, viewport: &Viewport) -> Vec<i32> {
        if let Some(mut quadtree) = self.quadtree.take() {
            let result;
            if let Some(ref mut actor) = self.get_mut(id) {
                let (rx, ry) = viewport.relative_point((actor.data().rect.x(),
                                                        actor.data().rect.y()));
                let rect = Rect::new_unwrap(rx,
                                            ry,
                                            actor.data().rect.width(),
                                            actor.data().rect.height());

                result = quadtree.retrieve(id, &rect);
            } else {
                result = vec![];
            }

            self.quadtree = Some(quadtree);
            return result;
        }

        vec![]
    }

    /// Temporarily remove an actor to appease borrow checker
    pub fn temp_remove(&mut self, id: i32) -> Option<Box<Actor<Type, Message>>> {
        self.temporary = Some(id);
        self.actors.remove(&id)
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<Actor<Type, Message>>> {
        self.actors.get_mut(&id)
    }

    /// Reinsert a temporarily removed actor
    pub fn temp_reinsert(&mut self, id: i32, actor: Box<Actor<Type, Message>>) {
        if let Some(temp_id) = self.temporary {
            // only insert the actor if it is the temporary one
            if id == temp_id {
                self.actors.insert(id, actor);
            }
        }
    }
}
