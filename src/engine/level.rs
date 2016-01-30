use engine::view::Actor;
use engine::viewport::Viewport;
use sdl2::render::Renderer;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Generates the level character token to actor configurations
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

pub fn load_level<F>(path: &str,
                     actor_for_token: F,
                     viewport: &mut Viewport,
                     renderer: &mut Renderer,
                     fps: f64)
                     -> Vec<Box<Actor>>
    where F: Fn(char, &mut Renderer, f64) -> Box<Actor>
{
    let mut actors = Vec::new();

    let open_result = File::open(path);

    if let Ok(f) = open_result {
        let reader = BufReader::new(f);

        let mut x = 0;
        let mut y = 0;

        let mut has_player = false;

        for line in reader.lines() {
            for token in line.unwrap().chars() {
                if token != ' ' {
                    let mut actor = actor_for_token(token, renderer, fps);
                    actor.set_position((x, y));
                    actors.push(actor);

                    if token == 'P' {
                        has_player = true;
                        viewport.update((x, y));
                    }
                }

                x += GRID_SIZE;
            }

            x = 0;
            y += GRID_SIZE;
        }
        if !has_player {
            panic!(format!("Level at {} needs to have a player", path));
        }
    }

    actors
}
