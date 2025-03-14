use mold2d::{Context, View, ViewAction};
use sdl2::pixels::Color;
use std::error::Error;

/// Test view that should display a background
pub struct BackgroundView;

impl View for BackgroundView {
    fn render(&mut self, context: &mut Context, _elapsed: f64) -> Result<(), Box<dyn Error>> {
        // TODO: Draw background (right now just draws red as background)
        context.canvas.set_draw_color(Color::RGB(255, 0, 0));
        context.canvas.clear();
        Ok(())
    }

    fn update(&mut self, context: &mut Context, _elapsed: f64) -> Option<ViewAction> {
        if context.events.event_called_once("ESC") || context.events.event_called("QUIT") {
            return Some(ViewAction::Quit);
        }

        None
    }
}
