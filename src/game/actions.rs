/// Actor messages
pub enum ActorMessage {
    AddActor(char, (i32, i32)),
    RemoveActor(i32),
    SetViewport(i32, i32),
    DamageActor(i32, i32),
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
