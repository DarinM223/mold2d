pub mod actions;
pub mod actors;
pub mod views;

use crate::views::game_view::GameView;
use mold2d::event_loop;
use mold2d::Window;

fn main() {
    let window = Window {
        title: "Mold2d demo game",
        width: 1024,
        height: 600,
    };

    let result = event_loop::create_event_loop(window, |context| {
        Box::new(GameView::new("levels/level1.txt", context))
    });

    match result {
        Ok(_) => println!("Game exited successfully!"),
        Err(e) => println!("{}", e),
    }
}
