//
use crate::{keyboard::Keyboard, mouse::Mouse};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Device {
    Keyboard(PathBuf),
    Mouse(PathBuf),
    GamePad(PathBuf),
}

pub enum DeviceType {
    Keyboard(Keyboard, PathBuf),
    Mouse(Mouse, PathBuf),
    // GamePad(GamePad), // todo
}
impl DeviceType {
    pub fn connect(option_device: &Option<Device>) -> Option<Self> {
        match option_device {
            Some(Device::Keyboard(p)) => Keyboard::new(p, false)
                .map_or_else(|_| None, |k| Some(DeviceType::Keyboard(k, p.clone()))),
            Some(Device::Mouse(p)) => Mouse::new(p, false)
                .map_or_else(|_| None, |m| Some(DeviceType::Mouse(m, p.clone()))),
            _ => None?,
        }
    }
}

impl fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                DeviceType::Keyboard(_, p) => ("Keyboard", p),
                DeviceType::Mouse(_, p) => ("Mouse", p),
            }
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Devices {
    pub keyboards: Vec<PathBuf>,
    pub mice: Vec<PathBuf>,
    pub gamepads: Vec<PathBuf>,
}

impl Devices {
    pub fn new() -> std::io::Result<Devices> {
        let mut keyboards = vec![];
        let mut mice = vec![];
        let mut gamepads = vec![];

        for entry in fs::read_dir("/dev/input/by-id")? {
            let pathbuf = entry?.path();
            let last = pathbuf.iter().last().clone().unwrap().to_str().unwrap();
            if last.contains("event-mouse") {
                mice.push(pathbuf);
            } else if last.contains("event-kbd") {
                keyboards.push(pathbuf);
            } else if last.contains("event-joystic") {
                gamepads.push(pathbuf);
            }
        }

        Ok(Devices {
            keyboards,
            mice,
            gamepads,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_device() {
        let devices = Devices::new();
        println!("device {:?}", devices)
    }
}
