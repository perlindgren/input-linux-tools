//
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Device {
    Keyboard(PathBuf),
    Mouse(PathBuf),
    GamePad(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    GamePad,
}

impl From<Device> for DeviceType {
    fn from(device: Device) -> Self {
        match device {
            Device::Keyboard(_) => DeviceType::Keyboard,
            Device::Mouse(_) => DeviceType::Mouse,
            Device::GamePad(_) => DeviceType::GamePad,
        }
    }
}

impl From<DeviceType> for Device {
    fn from(device_type: DeviceType) -> Self {
        match device_type {
            DeviceType::Keyboard => Device::Keyboard(PathBuf::new()),
            DeviceType::Mouse => Device::Mouse(PathBuf::new()),
            DeviceType::GamePad => Device::GamePad(PathBuf::new()),
        }
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
        let mut joystics = vec![];

        for entry in fs::read_dir("/dev/input/by-id")? {
            let pathbuf = entry?.path();
            let last = pathbuf.iter().last().clone().unwrap().to_str().unwrap();
            if last.contains("event-mouse") {
                mice.push(pathbuf);
            } else if last.contains("event-kbd") {
                keyboards.push(pathbuf);
            } else if last.contains("event-joystic") {
                joystics.push(pathbuf);
            }
        }

        Ok(Devices {
            keyboards,
            mice,
            gamepads: joystics,
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
