use std::collections::HashMap;
use view::Actor;

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

    pub fn add(&mut self, actor: Box<Actor>) {
        self.actors.insert(self.next_id, actor);
        self.next_id += 1;
    }

    pub fn get_mut(&mut self, id: i32) -> Option<&mut Box<Actor>> {
        self.actors.get_mut(&id)
    }
}
