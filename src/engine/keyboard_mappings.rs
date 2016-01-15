use std::collections::HashMap;

/// Loads keyboard mappings
/// Mappings are defined as keycode (integer) -> action (string)
#[derive(Debug)]
pub struct KeyboardMappings {
    key_map: HashMap<i32, String>,
}

#[derive(PartialEq, Eq)]
enum MappingState {
    Keycode,
    Action(i32),
}

impl KeyboardMappings {
    /// Creates a new keyboard given a path to a 
    /// keyboard mapping string
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

        return keyboard_mappings;
    }

    /// Returns the action command given a keycode
    pub fn get_action(&self, keycode: i32) -> Option<&String> {
        return self.key_map.get(&keycode);
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
