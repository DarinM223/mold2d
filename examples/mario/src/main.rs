#[macro_use(block)]
extern crate mold2d;

extern crate cpuprofiler;
extern crate sdl2;

pub mod actions;
pub mod actors;
pub mod views;

use cpuprofiler::PROFILER;
use mold2d::Window;
use mold2d::event_loop;
use views::game_view::GameView;

fn main() {
    let window = Window {
        title: "Mold2d demo game",
        width: 1024,
        height: 600,
    };

    PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
    let result = event_loop::create_event_loop(window, |context| {
        Box::new(GameView::new("levels/level1.txt", context))
    });
    PROFILER.lock().unwrap().stop().unwrap();

    match result {
        Ok(_) => println!("Game exited successfully!"),
        Err(e) => println!("{}", e),
    }
}
