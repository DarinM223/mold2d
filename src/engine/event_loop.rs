use engine::context::{Context, Window};
use engine::events::Events;
use engine::frame_timer::{FrameAction, FrameTimer};
use engine::view::{Actor, View, ViewAction};
use sdl2;
use sdl2::SdlResult;
use sdl2_ttf;

/// Initializes SDL and creates the window and event loop
pub fn create_event_loop<F>(window: Window, init_view: F) -> SdlResult<()>
    where F: Fn(&mut Context) -> Box<View>
{
    let sdl_context = try!(sdl2::init());
    let video = try!(sdl_context.video());
    let mut timer = try!(sdl_context.timer());
    let _ttf_context = sdl2_ttf::init();

    let sdl_window = try!(video.window(window.title, window.width, window.height)
                               .position_centered()
                               .opengl()
                               .build());

    let mut game_context = Context::new(window,
                                        Events::new(try!(sdl_context.event_pump()), ""),
                                        try!(sdl_window.renderer().accelerated().build()));

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
            Some(ViewAction::Quit) => break,
            Some(ViewAction::ChangeView(view)) => curr_view = view,
            _ => {}
        }

        curr_view.render(&mut game_context, elapsed);

        // Render the scene
        game_context.renderer.present();
    }

    Ok(())
}
