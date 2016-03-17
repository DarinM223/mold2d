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
            quadtree: None,
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

    /// Builds the actor's quadtree given the viewport
    pub fn build_quadtree(&mut self, viewport: &Viewport) {
        let mut quadtree = Quadtree::new(Rect::new_unwrap(0,
                                                          0,
                                                          viewport.map_dimensions.0 as u32,
                                                          viewport.map_dimensions.1 as u32));
        let keys: Vec<i32> = self.actors.iter().map(|(id, _)| *id).collect();

        for key in keys {
            quadtree.insert(key, self);
        }

        self.quadtree = Some(quadtree);
    }

    /// Retrieves actors close to the given actor
    pub fn retrieve_actors(&mut self, id: i32, viewport: &Viewport) -> Vec<i32> {
        println!("Total actors: {}", self.actors.len());
        if let Some(mut quadtree) = self.quadtree.take() {
            let result: Vec<i32>;
            if let Some(mut actor) = self.temp_remove(id) {
                result = quadtree.retrieve(id, &actor.data().rect)
                                 .iter()
                                 .filter(|id| {
                                     let actor = self.actors.get_mut(id).unwrap();
                                     viewport.constrain_to_viewport(&actor.data().rect) != None
                                 })
                                 .map(|id| *id)
                                 .collect::<Vec<_>>();
                self.temp_reinsert(id, actor);
                println!("Retrieved: {} actors", result.len());
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
