use sdl2::render::Renderer;
use std::collections::HashMap;
use view::Actor;

/// Handler for creating an actor from a token character
pub type ActorFromToken<Type, Message> = Box<Fn(char, i32, (i32, i32), &mut Renderer)
                                                -> Box<Actor<Type, Message>>>;

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager<Type, Message> {
    pub actors: HashMap<i32, Box<Actor<Type, Message>>>,
    next_id: i32,
    temporary: Option<i32>,
    actor_gen: ActorFromToken<Type, Message>,
}

impl<Type, Message> ActorManager<Type, Message> {
    pub fn new(actor_gen: ActorFromToken<Type, Message>) -> ActorManager<Type, Message> {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
            temporary: None,
            actor_gen: actor_gen,
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, token: char, position: (i32, i32), renderer: &mut Renderer) {
        let actor = (self.actor_gen)(token, self.next_id, position, renderer);
        self.actors.insert(self.next_id, actor);
        self.next_id += 1;
    }

    /// Remove an actor from the actors
    pub fn remove(&mut self, id: i32) {
        if let Some(temp_id) = self.temporary {
            if id == temp_id {
                self.temporary = None;
            }
        }

        self.actors.remove(&id);
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
