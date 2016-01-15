use engine::keyboard_mappings::KeyboardMappings;
use sdl2::EventPump;
use sdl2::event::Event;
use std::collections::HashSet;

/// Handles keyboard events through SDL
pub struct Events {
    pump: EventPump,
    events: HashSet<String>,
    mappings: KeyboardMappings,
}

impl Events {
    pub fn new(pump: EventPump, mappings_path: &str) -> Events {
        Events {
            pump: pump,
            events: HashSet::new(),
            mappings: KeyboardMappings::from_file(mappings_path),
        }
    }

    /// Polls for events and stores them inside a HashSet
    pub fn poll(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    let action = match self.mappings.get_action(keycode.unwrap() as i32) {
                        Some(action) => action.clone(),
                        None => return,
                    };
                    self.events.insert(action);
                }
                Event::KeyUp { keycode, .. } => {
                    let action = match self.mappings.get_action(keycode.unwrap() as i32) {
                        Some(action) => action.clone(),
                        None => return,
                    };
                    self.events.remove(&action);
                }
                Event::Quit { .. } => {
                    self.events.insert("QUIT".to_owned());
                }
                _ => {}
            }
        }
    }

    /// True if the event is currently happening, False otherwise
    pub fn event_called(&self, event: &str) -> bool {
        self.events.contains(event)
    }
}
