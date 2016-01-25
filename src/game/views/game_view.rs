use engine::context::Context;
use engine::view::{Actor, ActorAction, View, ViewAction};
use game::actors::asteroid::Asteroid;
use game::actors::block::Block;
use rand::random;
use sdl2::pixels::Color;

pub enum GameState {
    Normal,
    Paused,
    Slowed(f64),
}

/// The main game view used for 
/// the actual gameplay
pub struct GameView {
    state: GameState,
    actors: Vec<Box<Actor>>,
}

impl GameView {
    pub fn new() -> GameView {
        GameView {
            state: GameState::Normal,
            actors: Vec::new(),
        }
    }
}

impl View for GameView {
    fn render(&mut self, context: &mut Context, elapsed: f64) {
        // start off with a black screen
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        // render contained actors
        for actor in &mut self.actors {
            actor.render(context, elapsed);
        }
    }

    fn update(&mut self, context: &mut Context, elapsed: f64) -> ViewAction {
        if context.events.event_called("QUIT") || context.events.event_called("ESC") {
            return ViewAction::Quit;
        }

        // Pressing enter adds random blocks
        // TODO: remove this after blocks are finished
        if context.events.event_called_once("ENTER") {
            let max_width = context.window.width - 100;
            let max_height = context.window.height - 100;

            let rand_x = (random::<u32>() % max_width) as i32 + 1;
            let rand_y = (random::<u32>() % max_height) as i32 + 1;
            let mut block = Block::new((rand_x, rand_y));

            self.actors.push(Box::new(block));
        }

        let mut actions = Vec::new();

        // update contained actors
        for actor in &mut self.actors {
            actions.push(actor.update(context, elapsed));
        }

        // apply actor actions to view
        for action in actions {
            match action {
                ActorAction::AddActor(actor) => self.actors.push(actor),
                _ => {}
            }
        }

        ViewAction::None
    }
}
