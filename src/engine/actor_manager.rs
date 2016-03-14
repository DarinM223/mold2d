use sdl2::render::Renderer;
use std::collections::{HashMap, HashSet};
use view::Actor;

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
    removed_actors: HashSet<i32>,
    actor_gen: Box<ActorFromToken<Type, Message>>,
}

impl<Type, Message> ActorManager<Type, Message> {
    pub fn new(actor_gen: Box<ActorFromToken<Type, Message>>) -> ActorManager<Type, Message> {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
            removed_actors: HashSet::new(),
            actor_gen: actor_gen,
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, token: char, position: (i32, i32), renderer: &mut Renderer) {
        let actor = self.actor_gen.actor_from_token(token, self.next_id, position, renderer);
        self.actors.insert(self.next_id, actor);
        self.next_id += 1;
    }

    /// Remove an actor from the actors
    pub fn remove(&mut self, id: i32) {
        self.removed_actors.insert(id);
        self.actors.remove(&id);
    }

    /// Temporarily remove an actor to appease borrow checker
    pub fn temp_remove(&mut self, id: i32) -> Option<Box<Actor<Type, Message>>> {
        self.actors.remove(&id)
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<Actor<Type, Message>>> {
        self.actors.get_mut(&id)
    }

    pub fn add_existing(&mut self, id: i32, actor: Box<Actor<Type, Message>>) {
        if !self.removed_actors.contains(&id) {
            self.actors.insert(id, actor);
        }
    }
}
