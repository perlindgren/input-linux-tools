use crate::{common::*, device::*};
use input_linux::{EvdevHandle, Event, Key, KeyEvent, KeyState, RelativeAxis, RelativeEvent};

use std::{collections::HashMap, convert::TryFrom, fs::File, os::fd::FromRawFd, path::PathBuf};

#[derive(Debug)]
enum KeyStatus {
    Released,
    Pressed,
    Autorepeat,
}

#[derive(Debug)]
pub struct KeyboardEvent {
    key: Key,
    status: KeyStatus,
}

pub struct Keyboard {
    evdev_handle: EvdevHandle<File>,
    pub ignore_autorepeat: bool,
    // _keys: HashMap<MouseButton, ButtonState>,
}

impl Keyboard {
    pub fn new(path: &PathBuf) -> Self {
        let file = File::open(path).unwrap();
        let nb = nonblock::NonBlockingReader::from_fd(file).unwrap();
        let fd = nb.as_raw(); // takes over the file
        let nb_file = unsafe { File::from_raw_fd(fd) };
        let evdev_handle = EvdevHandle::new(nb_file);
        // let _buttons = HashMap::new();
        Keyboard {
            evdev_handle,
            ignore_autorepeat: false, // _buttons,
        }
    }

    pub fn new_first_match(s: &str) -> Self {
        let devices = Devices::new().unwrap();

        let device_path = devices
            .keyboards
            .iter()
            .find(|m| m.to_str().unwrap().contains(s))
            .unwrap();
        Self::new(device_path)
    }

    pub fn read_event(&self) -> std::io::Result<Event> {
        self.evdev_handle.read_event()
    }

    pub fn read(&self) -> Option<KeyboardEvent> {
        Some(match self.evdev_handle.read_event() {
            Ok(Event::Key(KeyEvent {
                time: _, // maybe we should have this
                key,
                value,
                ..
            })) => KeyboardEvent {
                key,
                status: match value {
                    KeyState::RELEASED => KeyStatus::Released,
                    KeyState::PRESSED => KeyStatus::Pressed,
                    KeyState::AUTOREPEAT => {
                        if self.ignore_autorepeat {
                            None?
                        } else {
                            KeyStatus::Autorepeat
                        }
                    }
                    _ => None?,
                },
            },

            _ => None?,
        })
    }
}
