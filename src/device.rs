//
use crate::{keyboard::Keyboard, mouse::Mouse};
use log::*;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct Device {
    pub path: PathBuf,
    pub device_type: DeviceType,
    #[serde(skip)]
    pub evdev: Option<EvDev>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[default]
    Keyboard,
    Mouse,
    GamePad, // TODO
}

#[derive()]
pub enum EvDev {
    Keyboard(Keyboard),
    Mouse(Mouse),
    GamePad, // TODO
}

impl Device {
    pub fn connect(&mut self) {
        match self.device_type {
            DeviceType::Keyboard => {
                self.evdev = Keyboard::new(&self.path, false)
                    .map_or_else(|_| None, |k| Some(EvDev::Keyboard(k)))
            }
            DeviceType::Mouse => {
                self.evdev =
                    Mouse::new(&self.path, false).map_or_else(|_| None, |m| Some(EvDev::Mouse(m)));
            }
            DeviceType::GamePad => {
                debug!("not yet implemented");
            }
        }
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = if self.evdev.is_some() {
            "Device connected"
        } else {
            "Device unconnected"
        };
        f.debug_struct(s)
            .field("path", &self.path)
            .field("device_type", &self.device_type)
            .finish()
    }
}

#[derive(Debug, Default)]
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

// notice these are linux only tests, assuming user access to /dev/input
#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_device() {
        let devices = Devices::new();
        // We cannot make any meaningful assertion, so its more of compile/run test
        // Expected byhavir listing available devices, run:
        // cargo test -p input-linux-tools test_device -- --nocapture
        println!("device {:?}", devices)
    }

    #[test]
    fn test_debug() {
        let mut d = Device {
            path: PathBuf::from_str("/dev/input/mice").unwrap(),
            device_type: DeviceType::Keyboard,
            evdev: None,
        };
        assert_eq!(
            format!("{:?}", d),
            "Device unconnected { path: \"/dev/input/mice\", device_type: Keyboard }"
        );
        let mouse = Mouse::new(&d.path, false).unwrap();
        d.evdev = Some(EvDev::Mouse(mouse));
        assert_eq!(
            format!("{:?}", d),
            "Device connected { path: \"/dev/input/mice\", device_type: Keyboard }"
        );
    }
}
