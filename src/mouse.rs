use crate::{common::*, device::*};
use input_linux::{EvdevHandle, Event, Key, KeyEvent, RelativeAxis, RelativeEvent};

use std::{collections::HashMap, convert::TryFrom, fs::File, os::fd::FromRawFd, path::PathBuf};

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

impl From<Key> for MouseButton {
    fn from(key: Key) -> Self {
        match key {
            Key::ButtonLeft => MouseButton::Left,
            Key::ButtonRight => MouseButton::Right,
            Key::ButtonMiddle => MouseButton::Middle,
            Key::ButtonExtra => MouseButton::Back,
            Key::ButtonSide => MouseButton::Forward,
            other => MouseButton::Other(other as u16),
        }
    }
}

#[derive(Debug)]
pub enum MouseScrollUnit {
    Line,
    Pixel,
}

#[derive(Debug)]
pub struct MouseMotion {
    pub delta: Vec2,
}

#[derive(Debug)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl From<bool> for ButtonState {
    fn from(b: bool) -> Self {
        match b {
            true => ButtonState::Pressed,
            _ => ButtonState::Released,
        }
    }
}

impl ButtonState {
    /// Is this button pressed?
    pub fn is_pressed(&self) -> bool {
        matches!(self, ButtonState::Pressed)
    }
}

#[derive(Debug)]
pub struct MouseButtonInput {
    pub button: MouseButton,
    pub state: ButtonState,
    // pub window: Entity,
}

pub struct Mouse {
    evdev_handle: EvdevHandle<File>,
    _buttons: HashMap<MouseButton, ButtonState>,
}

#[derive(Debug)]
pub enum MouseEvent {
    ButtonEvent(MouseButtonInput),
    ScrollEvent,
    MotionEvent(MouseMotion),
}

impl Mouse {
    pub fn new(path: &PathBuf) -> Self {
        let file = File::open(path).unwrap();
        let nb = nonblock::NonBlockingReader::from_fd(file).unwrap();
        let fd = nb.as_raw(); // takes over the file
        let nb_file = unsafe { File::from_raw_fd(fd) };
        let evdev_handle = EvdevHandle::new(nb_file);
        let _buttons = HashMap::new();
        Mouse {
            evdev_handle,
            _buttons,
        }
    }

    pub fn new_first_match(s: &str) -> Self {
        let devices = Devices::new().unwrap();

        let device_path = devices
            .mice
            .iter()
            .find(|m| m.to_str().unwrap().contains(s))
            .unwrap();
        Self::new(device_path)
    }

    pub fn read_event(&self) -> std::io::Result<Event> {
        self.evdev_handle.read_event()
    }

    pub fn read(&self) -> Option<MouseEvent> {
        Some(match self.evdev_handle.read_event() {
            Ok(Event::Key(KeyEvent {
                time: _, // maybe we should have this
                key,
                value,
                ..
            })) => {
                let button: MouseButton = key.into();
                MouseEvent::ButtonEvent(MouseButtonInput {
                    button,
                    state: value.is_pressed().into(),
                })
            }
            Ok(Event::Relative(RelativeEvent {
                time: _,
                axis,
                value,
                ..
            })) => match axis {
                RelativeAxis::X => MouseEvent::MotionEvent(MouseMotion {
                    delta: Vec2 {
                        x: value as f32,
                        y: 0.0,
                    },
                }),
                RelativeAxis::Y => MouseEvent::MotionEvent(MouseMotion {
                    delta: Vec2 {
                        x: 0.0,
                        y: value as f32,
                    },
                }),
                _ => None?,
            },
            _ => None?,
        })
    }
}
