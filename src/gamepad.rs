use crate::{device::*, nonblock::open_evdev};
use input_linux::{AbsoluteAxis, AbsoluteEvent, EvdevHandle, Event, Key, KeyEvent};
use std::{convert::From, fs::File, path::PathBuf};

pub struct GamePad {
    pub evdev_handle: EvdevHandle<File>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Button {
    pub key: Key,
    pub pressed: bool,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HatValue {
    NegOne,
    Zero,
    One,
}

impl From<i32> for HatValue {
    fn from(value: i32) -> Self {
        match value {
            -1 => HatValue::NegOne,
            1 => HatValue::One,
            _ => HatValue::Zero,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GamePadEvent {
    Button(Button),
    HatX(HatValue),
    HatY(HatValue),
    LeftStickX(f32),
    LeftStickY(f32),
    LeftBack(f32),
    RightStickX(f32),
    RightStickY(f32),
    RightBack(f32),
}

impl GamePad {
    pub fn new(path: &PathBuf, blocking: bool) -> std::io::Result<Self> {
        let evdev_handle = open_evdev(path, blocking)?;
        Ok(GamePad { evdev_handle })
    }

    pub fn new_first_match(s: &str, blocking: bool) -> std::io::Result<Self> {
        let devices = Devices::new()?;

        let device_path = devices
            .gamepads
            .iter()
            .find(|m| m.to_str().unwrap().contains(s))
            .unwrap();
        Self::new(device_path, blocking)
    }

    pub fn read_event(&self) -> std::io::Result<Event> {
        self.evdev_handle.read_event()
    }

    pub fn read(&self) -> Option<GamePadEvent> {
        loop {
            match self.evdev_handle.read_event() {
                Err(_) => None?, // no more events to read
                Ok(Event::Key(KeyEvent {
                    time: _, // maybe we should have this
                    key,
                    value,
                    ..
                })) => {
                    return Some(GamePadEvent::Button(Button {
                        key,
                        pressed: value.is_pressed(),
                    }));
                }
                Ok(Event::Absolute(AbsoluteEvent {
                    time: _,
                    axis,
                    value,
                    ..
                })) => match axis {
                    AbsoluteAxis::Hat0X => return Some(GamePadEvent::HatX(value.into())),
                    AbsoluteAxis::Hat0Y => return Some(GamePadEvent::HatY(value.into())),
                    AbsoluteAxis::X => {
                        return Some(GamePadEvent::LeftStickX(value as f32 / 32768.0))
                    }
                    AbsoluteAxis::Y => {
                        return Some(GamePadEvent::LeftStickY(value as f32 / 32768.0))
                    }
                    AbsoluteAxis::RX => {
                        return Some(GamePadEvent::RightStickX(value as f32 / 32768.0))
                    }
                    AbsoluteAxis::RY => {
                        return Some(GamePadEvent::RightStickY(value as f32 / 32768.0))
                    }
                    AbsoluteAxis::Z => return Some(GamePadEvent::LeftBack(value as f32 / 1024.0)),
                    AbsoluteAxis::RZ => {
                        return Some(GamePadEvent::RightBack(value as f32 / 1024.0))
                    }
                    _ => {
                        log::trace!("unexpeced axis {:?}, {:?}", axis, value); // skip this event and read next
                    }
                },

                Ok(event) => {
                    log::trace!("skip event: {:?}", event); // skip this event and read next
                }
            }
        }
    }
}
