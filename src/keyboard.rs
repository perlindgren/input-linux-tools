use crate::{device::*, nonblock::open_evdev};
use input_linux::{EvdevHandle, Event, Key, KeyEvent, KeyState};
use std::{/*collections::HashMap,*/ fs::File, path::PathBuf};

#[derive(Debug)]
pub enum KeyStatus {
    Released,
    Pressed,
    Autorepeat,
}

#[derive(Debug)]
pub struct KeyboardEvent {
    pub key: Key,
    pub status: KeyStatus,
}

pub struct Keyboard {
    evdev_handle: EvdevHandle<File>,
    pub ignore_autorepeat: bool,
    // _keys: HashMap<MouseButton, ButtonState>,
}

impl Keyboard {
    pub fn new(path: &PathBuf, blocking: bool) -> std::io::Result<Self> {
        let evdev_handle = open_evdev(path, blocking)?;
        Ok(Keyboard {
            evdev_handle,
            ignore_autorepeat: false,
        })
    }

    pub fn new_first_match(s: &str, blocking: bool) -> std::io::Result<Self> {
        let devices = Devices::new()?;
        let device_path = devices
            .keyboards
            .iter()
            .find(|m| m.to_str().unwrap().contains(s))
            .unwrap();
        Self::new(device_path, blocking)
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
