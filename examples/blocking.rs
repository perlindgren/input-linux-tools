use input_linux::{EvdevHandle, Event};

use std::{fs::File, os::fd::FromRawFd};
fn main() {
    let path =
        "/dev/input/by-id/usb-Kingston_Technology_Company_HyperX_Pulsefire_FPS_Pro-event-mouse";
    println!("waiting on input from: {:?}", path);
    let file = File::open(path).unwrap();
    let nb = nonblock::NonBlockingReader::from_fd(file).unwrap();
    std::mem::forget(nb);

    let file = File::open(path).unwrap();
    let ev_handle = EvdevHandle::new(file);

    loop {
        let r = ev_handle.read_event();
        match r {
            Ok(Event::Key(key_event)) => {
                println!("key : {:?}", key_event);
            }
            _ => {
                println!("--");
            }
        }
    }
}
