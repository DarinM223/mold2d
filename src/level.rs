use actor_manager::{ActorFromToken, ActorManager};
use context::Window;
use sdl2::render::Renderer;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use super::Actor;
use viewport::Viewport;

pub const GRID_SIZE: i32 = 40;

/// Loads a new level and returns an ActorManager with the loaded actors
pub fn load_level<A: Actor + ?Sized>(path: &str,
                                     actor_for_token: ActorFromToken<A>,
                                     renderer: &mut Renderer,
                                     window: &Window)
                                     -> io::Result<(ActorManager<A>, Viewport)> {
    let mut center_point = (0, 0);
    let mut manager = ActorManager::new(actor_for_token);

    File::open(path).and_then(|file| {
        let reader = BufReader::new(file);

        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut has_player = false;

        for line in reader.lines() {
            for token in try!(line).chars() {
                if token != ' ' {
                    manager.add(token, (x, y), renderer);

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
            return Err(io::Error::new(io::ErrorKind::Other,
                                      format!("Level at {} needs to have a player", path)));
        }

        let mut viewport = Viewport::new(window, (width, height));
        viewport.set_position(center_point);

        Ok((manager, viewport))
    })
}
