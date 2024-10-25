// gamepad, linux events
use input_linux_tools::gamepad::*;
use std::{thread, time};

fn main() {
    let gamepad = GamePad::new_first_match("event-joystick", true).unwrap();

    loop {
        if let Ok(e) = gamepad.read_event() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
