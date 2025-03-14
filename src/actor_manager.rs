use super::Actor;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorToken(pub char);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorPosition(pub i32, pub i32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ActorIndex {
    pub id: usize,
    pub generation: usize,
}

#[derive(Debug)]
pub struct NextIndex(ActorIndex);

impl NextIndex {
    pub fn index(&self) -> ActorIndex {
        self.0
    }
}

enum Slot<A: ?Sized> {
    Free { next_free: Option<usize> },
    Full { actor: Box<A>, generation: usize },
}

/// Manages all the actors for the game by hashing actors by id
pub struct ActorManager<A: Actor + ?Sized> {
    slots: Vec<Slot<A>>,
    free_top: Option<usize>,
    generation: usize,
    size: usize,
}

impl<A: Actor + ?Sized> Default for ActorManager<A> {
    fn default() -> ActorManager<A> {
        ActorManager::new()
    }
}

impl<A: Actor + ?Sized> ActorManager<A> {
    pub fn new() -> ActorManager<A> {
        ActorManager {
            slots: Vec::new(),
            free_top: None,
            generation: 0,
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> ActorManager<A> {
        ActorManager {
            slots: Vec::with_capacity(capacity),
            free_top: None,
            generation: 0,
            size: 0,
        }
    }

    /// Removes the next index from the manager and returns it.
    pub fn next_index(&mut self) -> NextIndex {
        let id = match self.free_top.take() {
            Some(top) => top,
            None => self.slots.len(),
        };
        NextIndex(ActorIndex {
            id,
            generation: self.generation,
        })
    }

    /// Adds a new actor into the manager.
    ///
    /// The index passed in must be the result of next_index().
    pub fn add(&mut self, NextIndex(index): NextIndex, actor: Box<A>) {
        let actor_slot = Slot::Full {
            actor,
            generation: index.generation,
        };

        let id = index.id;
        if id == self.slots.len() {
            self.slots.push(actor_slot);
        } else {
            if let Some(Slot::Free { next_free }) = self.slots.get(id) {
                self.free_top = *next_free;
            }
            self.slots[id] = actor_slot;
        }
        self.size += 1;
    }

    /// Removes an actor from the manager.
    pub fn remove(&mut self, index: ActorIndex) {
        let id = index.id;

        match self.slots.get(id) {
            Some(Slot::Free { .. }) => return,
            Some(Slot::Full { generation, .. }) if *generation != index.generation => return,
            None => return,
            _ => {}
        }

        self.slots[id] = Slot::Free {
            next_free: self.free_top.take(),
        };
        self.free_top = Some(id);
        self.generation += 1;
        self.size -= 1;
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut A> {
        self.slots.iter_mut().filter_map(|slot| match slot {
            Slot::Full { actor, .. } => Some(&mut **actor),
            _ => None,
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ActorIndex, &mut A)> {
        self.slots
            .iter_mut()
            .enumerate()
            .filter_map(|(id, slot)| match slot {
                Slot::Full { actor, generation } => Some((
                    ActorIndex {
                        id,
                        generation: *generation,
                    },
                    &mut **actor,
                )),
                _ => None,
            })
    }

    /// Get a mutable reference to an actor given the id
    pub fn get_mut(&mut self, index: ActorIndex) -> Option<&mut A> {
        match self.slots.get_mut(index.id) {
            Some(Slot::Full { actor, generation }) if *generation == index.generation => {
                Some(&mut **actor)
            }
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ActorData;
    use crate::collision::CollisionSide;
    use crate::context::Context;
    use crate::vector::PositionChange;
    use crate::viewport::Viewport;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;
    use std::error::Error;

    #[derive(Debug, Clone, PartialEq)]
    struct TestActor(ActorIndex);
    impl Actor for TestActor {
        type Type = ();
        type Message = ();

        fn handle_message(&mut self, _message: &()) -> () {
            ()
        }
        fn collides_with(&mut self, _other: &ActorData<()>) -> Option<CollisionSide> {
            None
        }
        fn update(&mut self, _context: &mut Context, _elapsed: f64) -> PositionChange {
            PositionChange {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            }
        }
        fn render(
            &mut self,
            _context: &mut Context,
            _viewport: &mut Viewport,
            _elapsed: f64,
        ) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn data(&mut self) -> ActorData<Self::Type> {
            ActorData {
                index: self.0,
                state: 0,
                damage: 0,
                collision_filter: 0,
                resolves_collisions: false,
                rect: Rect::new(0, 0, 0, 0),
                bounding_box: None,
                actor_type: (),
            }
        }
    }

    fn actor_from_token(
        _token: ActorToken,
        index: ActorIndex,
        _position: ActorPosition,
        _canvas: &mut Canvas<Window>,
    ) -> Box<dyn Actor<Type = (), Message = ()>> {
        Box::new(TestActor(index))
    }

    #[test]
    fn test_insert_and_remove() {
        let mut manager = ActorManager::with_capacity(100);
        let mut indexes = Vec::new();
        for _ in 0..100 {
            let next_index = manager.next_index();
            let index = next_index.index();
            manager.add(next_index, Box::new(TestActor(index)));
            indexes.push(index);
        }

        for i in indexes.iter().take(50) {
            assert!(manager.get_mut(*i).is_some());
            manager.remove(*i);
            assert_eq!(manager.get_mut(*i), None);
        }

        for _ in 0..50 {
            let next_index = manager.next_index();
            let index = next_index.index();
            manager.add(next_index, Box::new(TestActor(index)));
        }

        assert_eq!(manager.len(), 100);
        assert_eq!(manager.capacity(), 100);

        let mut count = 0;
        manager.values_mut().for_each(|_| count += 1);
        assert_eq!(count, 100);
    }

    #[test]
    fn test_stale_index() {
        let mut manager = ActorManager::new();

        let index;
        let next_index = manager.next_index();
        index = next_index.index();
        manager.add(next_index, Box::new(TestActor(index)));
        assert!(manager.get_mut(index).is_some());

        // After replacing the value, you shouldn't be able to read
        // from the old index.
        manager.remove(index);
        let next_index = manager.next_index();
        let new_index = next_index.index();
        assert_eq!(index.id, new_index.id);
        manager.add(next_index, Box::new(TestActor(new_index)));

        assert!(manager.get_mut(new_index).is_some());
        assert_eq!(manager.get_mut(index), None);
    }
}
