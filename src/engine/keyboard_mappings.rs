use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

// TODO(DarinM223): edit this as necessary for default keycodes
pub const KEYBOARD_DEFAULTS: &'static str = r#"
27 ESC 
13 ENTER 
32 SPACE
1073741906 UP 
1073741905 DOWN 
1073741904 LEFT 
1073741903 RIGHT
"#;

#[derive(PartialEq, Eq)]
enum MappingState {
    Keycode,
    Action(i32),
}

/// Loads keyboard mappings
/// Mappings are defined as keycode (integer) -> action (string)
#[derive(Debug)]
pub struct KeyboardMappings {
    key_map: HashMap<i32, String>,
}

impl KeyboardMappings {
    /// Creates a new keyboard mapper given a keyboard mapping string
    pub fn new(mappings: &str) -> KeyboardMappings {
        let mapping_str = mappings.to_owned();
        let token_stream: Vec<_> = mapping_str.split(|x| (x == ' ') || (x == '\n'))
                                              .filter(|s| !s.trim().is_empty())
                                              .collect();

        let mut keyboard_mappings = KeyboardMappings { key_map: HashMap::new() };
        let mut state = MappingState::Keycode;

        for token in &token_stream {
            match state {
                MappingState::Keycode => {
                    let keycode = token.parse::<i32>().unwrap();
                    state = MappingState::Action(keycode);
                }
                MappingState::Action(keycode) => {
                    let action = token.clone().to_owned();

                    keyboard_mappings.key_map.insert(keycode, action);
                    state = MappingState::Keycode;
                }
            }
        }

        keyboard_mappings
    }

    /// Creates a new keyboard mapper given a path to a keyboard mapping file
    pub fn from_file(path: &str) -> KeyboardMappings {
        // try to open file, but if file doesn't exist just use defaults
        match File::open(path) {
            Ok(ref mut f) => {
                let mut mappings = String::new();
                f.read_to_string(&mut mappings);
                KeyboardMappings::new(&mappings[..])
            }
            _ => KeyboardMappings::new(KEYBOARD_DEFAULTS),
        }
    }

    /// Returns the action command given a keycode
    pub fn get_action(&self, keycode: i32) -> Option<&String> {
        self.key_map.get(&keycode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic() {
        let s = "10 hello 11 world";
        let mappings = KeyboardMappings::new(s);

        assert_eq!(*mappings.get_action(10).unwrap(), "hello".to_owned());
        assert_eq!(*mappings.get_action(11).unwrap(), "world".to_owned());
    }

    #[test]
    fn parse_newlines() {
        let s = "10 hello\n \n 11 world\n";
        let mappings = KeyboardMappings::new(s);

        assert_eq!(*mappings.get_action(10).unwrap(), "hello".to_owned());
        assert_eq!(*mappings.get_action(11).unwrap(), "world".to_owned());
    }

    #[test]
    fn parse_unbalanced() {
        // should drop unbalanced tokens
        let s = "10 hello 11 world 12";

        let mappings = KeyboardMappings::new(s);
        assert_eq!(*mappings.get_action(10).unwrap(), "hello".to_owned());
        assert_eq!(*mappings.get_action(11).unwrap(), "world".to_owned());
        assert_eq!(mappings.get_action(12), None);
    }
}
