use iced::keyboard::KeyCode;
use lazy_static::lazy_static;
use nes_core::controller::ControllerState;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::{Arc, RwLock};

use crate::emulator::Message;

const INPUTS_LIST: &[Input] = {
    use Input::*;
    &[
        ButtonA,
        ButtonB,
        ButtonStart,
        ButtonSelect,
        ButtonUp,
        ButtonDown,
        ButtonLeft,
        ButtonRight,
        Pause,
        VolumeUp,
        VolumeDown,
    ]
};

lazy_static! {
    static ref INPUT_HANDLER: Arc<RwLock<InputHandler>> =
        Arc::new(RwLock::new(InputHandler::default()));
}

/// Handles keyboard events using a global input handler
pub(super) fn event_handler(
    event: iced::Event,
    _status: iced::event::Status,
) -> Option<super::Message> {
    match event {
        iced::Event::Keyboard(event) => match event {
            iced::keyboard::Event::KeyPressed { key_code, .. } => {
                let input = INPUT_HANDLER.read().unwrap().translate_keypresses(key_code);
                input.and_then(Input::msg_on_press)
            }
            iced::keyboard::Event::KeyReleased { key_code, .. } => {
                let input = INPUT_HANDLER.read().unwrap().translate_keypresses(key_code);
                input.and_then(Input::msg_on_release)
            }
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Input {
    ButtonA,
    ButtonB,
    ButtonStart,
    ButtonSelect,
    ButtonUp,
    ButtonDown,
    ButtonLeft,
    ButtonRight,

    Pause,
    VolumeUp,
    VolumeDown,
}

impl Input {
    fn msg_on_press(self) -> Option<super::Message> {
        use Input::*;

        match self {
            ButtonA => Some(Message::ControllerButtonPressed(ControllerState::A)),
            ButtonB => Some(Message::ControllerButtonPressed(ControllerState::B)),
            ButtonStart => Some(Message::ControllerButtonPressed(ControllerState::START)),
            ButtonSelect => Some(Message::ControllerButtonPressed(ControllerState::SELECT)),
            ButtonUp => Some(Message::ControllerButtonPressed(ControllerState::UP)),
            ButtonDown => Some(Message::ControllerButtonPressed(ControllerState::DOWN)),
            ButtonLeft => Some(Message::ControllerButtonPressed(ControllerState::LEFT)),
            ButtonRight => Some(Message::ControllerButtonPressed(ControllerState::RIGHT)),

            Pause => Some(Message::TogglePause),
            VolumeUp => Some(Message::VolumeChange(50)),
            VolumeDown => Some(Message::VolumeChange(-50)),
        }
    }

    fn msg_on_release(self) -> Option<super::Message> {
        use Input::*;

        match self {
            ButtonA => Some(Message::ControllerButtonReleased(ControllerState::A)),
            ButtonB => Some(Message::ControllerButtonReleased(ControllerState::B)),
            ButtonStart => Some(Message::ControllerButtonReleased(ControllerState::START)),
            ButtonSelect => Some(Message::ControllerButtonReleased(ControllerState::SELECT)),
            ButtonUp => Some(Message::ControllerButtonReleased(ControllerState::UP)),
            ButtonDown => Some(Message::ControllerButtonReleased(ControllerState::DOWN)),
            ButtonLeft => Some(Message::ControllerButtonReleased(ControllerState::LEFT)),
            ButtonRight => Some(Message::ControllerButtonReleased(ControllerState::RIGHT)),

            _ => None,
        }
    }
}

type DuplicateInputs = Vec<Input>;

#[derive(Debug)]
pub struct InputHandler {
    /// Map assigning a keyboard button to each input. This is what the user sees on the input config screen.
    forward_map: HashMap<Input, KeyCode>,
    /// Map to quickly convert keypresses to inputs. Detects duplicated keymaps.
    reverse_map: HashMap<KeyCode, Result<Input, DuplicateInputs>>,
}

#[allow(unused)]
impl InputHandler {
    pub fn from_keymaps(keymaps: HashMap<Input, KeyCode>) -> Self {
        let mut s = InputHandler {
            forward_map: keymaps,
            reverse_map: HashMap::new(),
        };

        // Build up the reverse map from scratch
        for (&input, &key) in s.forward_map.iter() {
            Self::reverse_map_bind(&mut s.reverse_map, input, key);
        }

        s
    }

    pub fn bind_input(&mut self, input: Input, key: KeyCode) {
        let old_binding = self.forward_map.insert(input, key);
        if let Some(old_key) = old_binding {
            // Remove the old keymap from the reverse map, if it exists
            Self::reverse_map_unbind(&mut self.reverse_map, input, old_key);
        }
        // Add the new key to the reverse map
        Self::reverse_map_bind(&mut self.reverse_map, input, key);
    }

    pub fn unbind_input(&mut self, input: Input) {
        if let Some(old_key) = self.forward_map.remove(&input) {
            Self::reverse_map_unbind(&mut self.reverse_map, input, old_key);
        }
    }

    pub fn translate_keypresses(&self, key: KeyCode) -> Option<Input> {
        self.reverse_map
            .get(&key)
            .map(Result::as_ref)
            .and_then(Result::ok) // ignore duplicated keymaps
            .copied()
    }

    pub fn keymaps(&self) -> impl Iterator<Item = KeyMap> + '_ {
        INPUTS_LIST.iter().map(|&i| {
            let key = self.forward_map.get(&i).copied();
            let is_duplicate = key
                .and_then(|k| self.reverse_map.get(&k).map(Result::is_err))
                .unwrap_or(false);
            KeyMap {
                input: i,
                key,
                is_duplicate,
            }
        })
    }

    fn reverse_map_bind(
        reverse_map: &mut HashMap<KeyCode, Result<Input, DuplicateInputs>>,
        input: Input,
        key: KeyCode,
    ) {
        match reverse_map.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(Ok(input));
            }
            Entry::Occupied(mut entry) => {
                // If this key is already bound, then we have to mark this key as duplicated.
                let entry = entry.get_mut();
                match entry {
                    Ok(other_input) => {
                        *entry = Err(vec![*other_input, input]);
                    }
                    Err(dupes) => {
                        dupes.push(input);
                    }
                }
            }
        }
    }

    fn reverse_map_unbind(
        reverse_map: &mut HashMap<KeyCode, Result<Input, DuplicateInputs>>,
        input: Input,
        old_key: KeyCode,
    ) {
        match reverse_map.entry(old_key) {
            // If this function is run on a keymap that does not exist then something has gone very wrong.
            Entry::Vacant(_entry) => unreachable!(),
            Entry::Occupied(mut entry) => match entry.get_mut() {
                Ok(_) => {
                    // If this keymap is not duplicated, then it's simple to remove.
                    let _ = entry.remove();
                }
                Err(dupes) => {
                    dupes.swap_remove(dupes.iter().position(|&x| x == input).unwrap());
                    // If it is duplicated, we have to check if unbinding this key un-duplicates it.
                    if dupes.len() == 1 {
                        let new_input = dupes[0]; // this satisfies the borrow checker
                        let _ = entry.insert(Ok(new_input));
                    }
                }
            },
        };
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        let keymaps = HashMap::from([
            (Input::ButtonA, KeyCode::Z),
            (Input::ButtonB, KeyCode::X),
            (Input::ButtonStart, KeyCode::G),
            (Input::ButtonSelect, KeyCode::H),
            (Input::ButtonUp, KeyCode::Up),
            (Input::ButtonDown, KeyCode::Down),
            (Input::ButtonLeft, KeyCode::Left),
            (Input::ButtonRight, KeyCode::Right),
            (Input::Pause, KeyCode::P),
            (Input::VolumeUp, KeyCode::Equals),
            (Input::VolumeDown, KeyCode::Minus),
        ]);
        InputHandler::from_keymaps(keymaps)
    }
}

#[derive(Debug)]
pub struct KeyMap {
    pub input: Input,
    pub key: Option<KeyCode>,
    pub is_duplicate: bool,
}
