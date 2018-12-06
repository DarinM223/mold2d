use super::Actor;
use crate::actor_manager::{ActorIndex, ActorManager, ActorPosition, ActorToken};
use crate::context::Window;
use crate::viewport::Viewport;
use sdl2::render::Renderer;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub const GRID_SIZE: i32 = 40;

/// Loads a new level and returns an ActorManager with the loaded actors
pub fn load_level<A, F>(
    path: &str,
    actor_for_token: F,
    renderer: &mut Renderer,
    window: &Window,
) -> io::Result<(ActorManager<A>, Viewport)>
where
    A: Actor + ?Sized,
    F: Fn(ActorToken, ActorIndex, ActorPosition, &mut Renderer) -> Box<A>,
{
    let mut center_point = (0, 0);
    let mut manager = ActorManager::new();

    File::open(path).and_then(|file| {
        let reader = BufReader::new(file);

        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut has_player = false;

        for line in reader.lines() {
            for token in line?.chars() {
                if token != ' ' {
                    let next_index = manager.next_index();
                    let actor = actor_for_token(
                        ActorToken(token),
                        next_index.index(),
                        ActorPosition(x, y),
                        renderer,
                    );
                    manager.add(next_index, actor);

                    if token == 'P' {
                        has_player = true;
                        center_point = (x, y);
                    }
                }

                x += GRID_SIZE;
            }

            width = x;
            x = 0;
            y += GRID_SIZE;
        }

        let (width, height) = (width, y);

        if !has_player {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Level at {} needs to have a player", path),
            ));
        }

        let mut viewport = Viewport::new(window, (width, height));
        viewport.set_position(center_point);

        Ok((manager, viewport))
    })
}
