use sdl2::render::Renderer;
use std::collections::HashMap;
use view::Actor;

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager<Type, Message> {
    next_id: i32,
    pub actors: HashMap<i32, Box<Actor<Type, Message>>>,
    actor_gen: Box<ActorFromToken<Type, Message>>,
}

pub trait ActorFromToken<Type, Message> {
    /// Returns an actor given a token
    fn actor_from_token(&self,
                        token: char,
                        id: i32,
                        position: (i32, i32),
                        renderer: &mut Renderer)
                        -> Box<Actor<Type, Message>>;
}

impl<Type, Message> ActorManager<Type, Message> {
    pub fn new(actor_gen: Box<ActorFromToken<Type, Message>>) -> ActorManager<Type, Message> {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
            actor_gen: actor_gen,
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, token: char, position: (i32, i32), renderer: &mut Renderer) {
        let actor = self.actor_gen.actor_from_token(token, self.next_id, position, renderer);
        self.actors.insert(self.next_id, actor);
        self.next_id += 1;
    }

    pub fn remove(&mut self, id: i32) {
        self.actors.remove(&id);
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<Actor<Type, Message>>> {
        self.actors.get_mut(&id)
    }
}
