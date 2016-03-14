/// Actions for an actor to process
pub enum ActorAction {
    DamageActor(i32),
}

/// Actor messages
pub enum ActorMessage {
    AddActor(char, (i32, i32)),
    RemoveActor(i32),
    SetViewport(i32, i32),
    ActorAction(i32, ActorAction),
    MultipleMessages(Vec<Box<ActorMessage>>),
    PlayerDied,
    None,
}

/// Actor types
#[derive(Clone, PartialEq)]
pub enum ActorType {
    Item,
    Block,
    Player,
    Enemy,
}
