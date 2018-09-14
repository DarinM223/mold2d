use super::Actor;
use sdl2::render::Renderer;
use std::collections::HashMap;

/// Handler for creating an actor from a token character
pub type ActorFromToken<A> = Box<Fn(char, i32, (i32, i32), &mut Renderer) -> Box<A>>;

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager<A: Actor + ?Sized> {
    pub actors: HashMap<i32, Box<A>>,
    next_id: i32,
    actor_gen: ActorFromToken<A>,
}

impl<A: Actor + ?Sized> ActorManager<A> {
    pub fn new(actor_gen: ActorFromToken<A>) -> ActorManager<A> {
        ActorManager {
            next_id: 0,
            actors: HashMap::new(),
            actor_gen,
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
        self.actors.remove(&id);
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<A>> {
        self.actors.get_mut(&id)
    }

    /// Attempts to send a message to an actor and returns either
    /// the response or a given default message if the actor can't be found
    pub fn apply_message(
        &mut self,
        actor_id: i32,
        msg: &A::Message,
        none: A::Message,
    ) -> A::Message {
        self.actors
            .get_mut(&actor_id)
            .map_or(none, |actor| actor.handle_message(msg))
    }
}
