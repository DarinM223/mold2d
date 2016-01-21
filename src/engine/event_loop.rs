use engine::context::Context;
use engine::events::Events;
use engine::view::{Actor, View, ViewAction};
use game::actors::asteroid::Asteroid;
use sdl2;
use sdl2::TimerSubsystem;
use sdl2_ttf;

const FRAME_INTERVAL: u32 = 1000 / 60;

enum FrameAction {
    /// Block the event loop 
    Delay,
    /// Continue with the elapsed time
    Continue(f64),
}

struct FrameTimer<'a> {
    timer: &'a mut TimerSubsystem,
    before: u32,
    last_second: u32,
    fps: u16,
    debug: bool,
}

impl<'a> FrameTimer<'a> {
    pub fn new(timer: &'a mut TimerSubsystem, debug: bool) -> FrameTimer<'a> {
        FrameTimer {
            before: timer.ticks(),
            last_second: timer.ticks(),
            timer: timer,
            fps: 0u16,
            debug: debug,
        }
    }

    /// Call this function every frame to limit the frames to a 
    /// certain FPS
    pub fn on_frame(&mut self) -> FrameAction {
        let now = self.timer.ticks();
        let time_change = now - self.before;
        let elapsed = time_change as f64 / 1000.0;

        if time_change < FRAME_INTERVAL {
            self.timer.delay(FRAME_INTERVAL - time_change);
            return FrameAction::Delay;
        }

        self.before = now;
        self.fps += 1;

        if now - self.last_second > 1000 {
            if self.debug {
                println!("FPS: {}", self.fps);
            }

            self.last_second = now;
            self.fps = 0;
        }

        FrameAction::Continue(elapsed)
    }
}

/// Represents a SDL window to render
pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

/// Initializes SDL and creates the window and event loop
pub fn create_event_loop<F>(window: Window, init_view: F)
    where F: Fn(&mut Context) -> Box<View>
{
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _ttf_context = sdl2_ttf::init();

    let window = video.window(window.title.as_str(), window.width, window.height)
                      .position_centered()
                      .opengl()
                      .build()
                      .unwrap();

    let mut game_context = Context::new(Events::new(sdl_context.event_pump().unwrap(), ""),
                                        window.renderer().accelerated().build().unwrap());

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
