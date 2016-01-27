use engine::view::Actor;
use game::actors::asteroid::Asteroid;
use game::actors::block::Block;
use sdl2::render::Renderer;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Generates the level character token to actor configurations
macro_rules! level_token_config {
    ( $( $token:expr => $actor:ident ),* ) => {
        pub fn actor_for_token(token: char, 
                               renderer: &mut Renderer, 
                               fps: f64) -> Box<Actor> {
            match token {
                $( $token => Box::new($actor::new(renderer, fps)), )*
                _ => unreachable!(),
            }
        }
    }
}

level_token_config! {
    '+' => Asteroid,
    '=' => Block
}

const GRID_SIZE: i32 = 20;

pub fn load_level(path: &str, renderer: &mut Renderer, fps: f64) -> Vec<Box<Actor>> {
    let mut actors = Vec::new();

    let open_result = File::open(path);

    if let Ok(f) = open_result {
        let reader = BufReader::new(f);

        let mut x = 0;
        let mut y = 0;

        for line in reader.lines() {
            for token in line.unwrap().chars() {
                if token != ' ' {
                    let mut actor = actor_for_token(token, renderer, fps);
                    actor.set_position((x, y));
                    actors.push(actor);
                }

                x += GRID_SIZE; 
            }

            y += GRID_SIZE;
        }
    }

    actors
}
