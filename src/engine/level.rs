use actor_manager::ActorManager;
use context::Window;
use sdl2::render::Renderer;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use view::Actor;
use viewport::Viewport;

/// Generates the level character token to actor configurations
#[macro_export]
macro_rules! level_token_config {
    ( $( $token:expr => $actor:ident ),* ) => {
        pub fn actor_for_token(token: char, 
                               renderer: &mut ::sdl2::render::Renderer, 
                               fps: f64) -> Box<::engine::view::Actor> {
            match token {
                $( $token => Box::new($actor::new(renderer, fps)), )*
                _ => unreachable!(),
            }
        }
    }
}

pub const GRID_SIZE: i32 = 40;

/// Loads a new level and returns an ActorManager with the loaded actors
pub fn load_level<F>(path: &str,
                     actor_for_token: F,
                     renderer: &mut Renderer,
                     window: &Window,
                     fps: f64)
                     -> io::Result<(ActorManager, Viewport)>
    where F: Fn(char, &mut Renderer, f64) -> Box<Actor>
{
    let mut center_point = (0, 0);
    let mut manager = ActorManager::new();

    let open_result = File::open(path);

    if let Ok(f) = open_result {
        let reader = BufReader::new(f);

        let mut x = 0;
        let mut y = 0;

        let mut width = 0;

        let mut has_player = false;

        for line in reader.lines() {
            for token in try!(line).chars() {
                if token != ' ' {
                    let mut actor = actor_for_token(token, renderer, fps);
                    actor.set_position((x, y));
                    manager.add(actor);

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
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData,
                           format!("File could not be opened")))
    }
}
