use sdl2::TimerSubsystem;

const FRAME_INTERVAL: u32 = 1000 / 60;

pub enum FrameAction {
    /// Block the event loop 
    Delay,
    /// Continue with the elapsed time
    Continue(f64),
}

/// Used by the event loop to limit frames to a maximum of a certain FPS
pub struct FrameTimer<'a> {
    pub fps: u16,
    timer: &'a mut TimerSubsystem,
    before: u32,
    last_second: u32,
    debug: bool,
}

/// Delays the event loop to match a certain FPS
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
