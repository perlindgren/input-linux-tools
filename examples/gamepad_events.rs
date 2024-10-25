// gamepad, events
use input_linux_tools::gamepad::*;
use std::{thread, time};

fn main() {
    let gamepad = GamePad::new_first_match("event-joystic", true).unwrap();

    loop {
        if let Some(e) = gamepad.read() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
