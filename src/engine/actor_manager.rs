use std::collections::HashMap;
use view::Actor;

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager {
    next_id: i32,
    pub actors: HashMap<i32, Box<Actor>>,
}

impl ActorManager {
    pub fn new() -> ActorManager {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, actor: Box<Actor>) {
        self.actors.insert(self.next_id, actor);
        self.next_id += 1;
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<Actor>> {
        self.actors.get_mut(&id)
    }
}
