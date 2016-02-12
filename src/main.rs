extern crate engine;
extern crate game;

use engine::context::Window;
use engine::event_loop;
use game::views::game_view::GameView;

fn main() {
    let window = Window {
        title: "Trage - The troll rage game",
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
