use crate::{device::*, nonblock::open_evdev};
use input_linux::{EvdevHandle, Event, Key, KeyEvent, KeyState};
use std::{/*collections::HashMap,*/ fs::File, path::PathBuf};

#[derive(Debug, PartialEq)]
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
            ignore_autorepeat: true,
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
        loop {
            match self.evdev_handle.read_event() {
                Err(_) => None?, // no more events to read
                Ok(Event::Key(KeyEvent {
                    time: _, // maybe we should have this
                    key,
                    value,
                    ..
                })) => {
                    return Some(KeyboardEvent {
                        key,
                        status: match value {
                            KeyState::RELEASED => KeyStatus::Released,
                            KeyState::PRESSED => KeyStatus::Pressed,
                            KeyState::AUTOREPEAT => {
                                if self.ignore_autorepeat {
                                    continue; // skip this event and read next
                                } else {
                                    KeyStatus::Autorepeat
                                }
                            }
                            _ => {
                                log::error!("unexpected value");
                                continue; // skip this event and read next
                            }
                        },
                    });
                }
                _ => {} // skip this event and read next
            }
        }
    }
}

#[cfg(test)]
mod test {
    use input_linux;
    use winit::{keyboard::PhysicalKey, platform::scancode::PhysicalKeyExtScancode};
    #[test]
    fn test_winit() {
        let linux_key = input_linux::Key::A;
        let physical_key: PhysicalKey = PhysicalKey::from_scancode(linux_key.code() as u32);
        println!("linux_key {:?}, physical_key {:?}", linux_key, physical_key);
    }
}
