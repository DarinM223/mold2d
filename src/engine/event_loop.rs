use engine::context::{Context, Window};
use engine::events::Events;
use engine::frame_timer::{FrameAction, FrameTimer};
use engine::view::{Actor, View, ViewAction};
use sdl2;
use sdl2_ttf;

/// Initializes SDL and creates the window and event loop
pub fn create_event_loop<F>(window: Window, init_view: F)
    where F: Fn(&mut Context) -> Box<View>
{
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _ttf_context = sdl2_ttf::init();

    let sdl_window = video.window(window.title.as_str(), window.width, window.height)
                          .position_centered()
                          .opengl()
                          .build()
                          .unwrap();

    let mut game_context = Context::new(window,
                                        Events::new(sdl_context.event_pump().unwrap(), ""),
                                        sdl_window.renderer().accelerated().build().unwrap());

    let mut frame_timer = FrameTimer::new(&mut timer, true);

    let mut curr_view = init_view(&mut game_context);

    loop {
        let elapsed;
        match frame_timer.on_frame() {
            FrameAction::Delay => continue,
            FrameAction::Continue(elpsed) => elapsed = elpsed,
        }

        game_context.events.poll();

        match curr_view.update(&mut game_context, elapsed) {
            ViewAction::Quit => break,
            _ => {}
        }

        curr_view.render(&mut game_context, elapsed);

        // Render the scene
        game_context.renderer.present();
    }
}
