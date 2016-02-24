use sdl2::render::Renderer;
use std::collections::HashMap;
use view::Actor;

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager {
    next_id: i32,
    pub actors: HashMap<i32, Box<Actor>>,
    actor_gen: Box<ActorFromToken>,
}

pub trait ActorFromToken {
    /// Returns an actor given a token
    fn actor_from_token(&self,
                        token: char,
                        id: i32,
                        position: (i32, i32),
                        renderer: &mut Renderer,
                        fps: f64)
                        -> Box<Actor>;
}

impl ActorManager {
    pub fn new(actor_gen: Box<ActorFromToken>) -> ActorManager {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
            actor_gen: actor_gen,
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, token: char, position: (i32, i32), renderer: &mut Renderer, fps: f64) {
        let actor = self.actor_gen.actor_from_token(token, self.next_id, position, renderer, fps);
        self.actors.insert(self.next_id, actor);
        self.next_id += 1;
    }

    pub fn remove(&mut self, id: i32) {
        self.actors.remove(&id);
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<Actor>> {
        self.actors.get_mut(&id)
    }
}
