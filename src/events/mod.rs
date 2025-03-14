mod keyboard_mappings;

use self::keyboard_mappings::KeyboardMappings;
use sdl2::EventPump;
use sdl2::event::Event;
use std::collections::HashSet;

/// Handles keyboard events through SDL
pub struct Events {
    pump: EventPump,
    events: HashSet<String>,
    once_events: HashSet<String>,
    mappings: KeyboardMappings,
}

impl Events {
    pub fn new(pump: EventPump, mappings_path: &str) -> Events {
        Events {
            pump,
            events: HashSet::new(),
            once_events: HashSet::new(),
            mappings: KeyboardMappings::from_file(mappings_path),
        }
    }

    /// Polls for events and stores them inside a HashSet
    #[inline]
    pub fn poll(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let action = match self.mappings.get_action(keycode.into_i32()) {
                        Some(action) => action,
                        None => return,
                    };

                    if self.events.contains(action) {
                        self.once_events.remove(action);
                    } else {
                        self.events.insert(action.clone());
                        self.once_events.insert(action.clone());
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    let action = match self.mappings.get_action(keycode.into_i32()) {
                        Some(action) => action.clone(),
                        None => return,
                    };
                    self.events.remove(&action);
                    self.once_events.remove(&action);
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

    /// True if the event is currently happening, False otherwise
    /// difference from event_called is that is only called once for an event
    pub fn event_called_once(&mut self, event: &str) -> bool {
        if self.once_events.contains(event) {
            self.once_events.remove(event);
            return true;
        }

        false
    }
}
