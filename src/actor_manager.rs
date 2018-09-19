use super::Actor;
use sdl2::render::Renderer;
use std::mem;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorId(usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorToken(pub char);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorPosition(pub i32, pub i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorIndex {
    id: ActorId,
    generation: usize,
}

/// Handler for creating an actor from a token character
pub type ActorFromToken<A> = Box<Fn(ActorToken, ActorId, ActorPosition, &mut Renderer) -> Box<A>>;

enum Slot<A: ?Sized> {
    Free { next_free: Option<usize> },
    Full { actor: Box<A>, generation: usize },
}

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager<A: Actor + ?Sized> {
    slots: Vec<Slot<A>>,
    free_top: Option<usize>,
    generation: usize,
    actor_gen: ActorFromToken<A>,
}

impl<A: Actor + ?Sized> ActorManager<A> {
    pub fn new(actor_gen: ActorFromToken<A>) -> ActorManager<A> {
        ActorManager {
            slots: Vec::new(),
            free_top: None,
            generation: 0,
            actor_gen,
        }
    }

    pub fn with_capacity(capacity: usize, actor_gen: ActorFromToken<A>) -> ActorManager<A> {
        ActorManager {
            slots: Vec::with_capacity(capacity),
            free_top: None,
            generation: 0,
            actor_gen,
        }
    }

    /// Add a new actor into the manager
    pub fn add(&mut self, token: ActorToken, position: ActorPosition, renderer: &mut Renderer) {
        let id = match self.free_top.take() {
            Some(top) => top,
            None => self.slots.len(),
        };
        let actor = (self.actor_gen)(token, ActorId(id), position, renderer);
        let actor_slot = Slot::Full {
            actor,
            generation: self.generation,
        };
        if id == self.slots.len() {
            self.slots.push(actor_slot);
        } else {
            if let Some(Slot::Free { next_free }) = self.slots.get(id) {
                self.free_top = *next_free;
            }
            mem::replace(&mut self.slots[id], actor_slot);
        }
    }

    /// Remove an actor
    pub fn remove(&mut self, index: ActorIndex) {
        let id = index.id.0;

        match self.slots.get(id) {
            Some(Slot::Free { .. }) => return,
            Some(Slot::Full { generation, .. }) if *generation != index.generation => return,
            _ => {}
        }

        mem::replace(
            &mut self.slots[id],
            Slot::Free {
                next_free: self.free_top.take(),
            },
        );
        self.free_top = Some(id);
        self.generation += 1;
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, index: ActorIndex) -> Option<&mut A> {
        match self.slots.get_mut(index.id.0) {
            Some(Slot::Full { actor, generation }) if *generation == index.generation => {
                Some(&mut **actor)
            }
            _ => None,
        }
    }

    /// Attempts to send a message to an actor and returns either
    /// the response or a given default message if the actor can't be found
    pub fn apply_message(
        &mut self,
        actor_id: ActorIndex,
        msg: &A::Message,
        none: A::Message,
    ) -> A::Message {
        self.get_mut(actor_id)
            .map_or(none, |actor| actor.handle_message(msg))
    }
}
