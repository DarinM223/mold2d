use crate::actions::{ActorMessage, ActorType};
use mold2d::block;

block! {
    actor_type: ActorType,
    actor_message: ActorMessage,
    blocks: {
        block {
            name: StartBlock,
            path: "assets/tiles.png",
            index: 0,
            width: 80,
            height: 80,
            sprites_in_row: 7,
            size: 40,
            collision_filter: 0b1111
        }

        block {
            name: GroundBlockTop,
            path: "assets/tiles.png",
            index: 14,
            width: 80,
            height: 80,
            sprites_in_row: 7,
            size: 40,
            collision_filter: 0b1111
        }

        block {
            name: GroundBlockMid,
            path: "assets/tiles.png",
            index: 21,
            width: 80,
            height: 80,
            sprites_in_row: 7,
            size: 40,
            collision_filter: 0b1110
        }

        block {
            name: StoneBlock,
            path: "assets/tiles.png",
            index: 7,
            width: 80,
            height: 80,
            sprites_in_row: 7,
            size: 40,
            collision_filter: 0b1111
        }
    }
}
