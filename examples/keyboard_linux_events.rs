// keyboard, linux events
use input_linux_tools::keyboard::*;
use std::{thread, time};

fn main() {
    // ignore autorepeat per defualt
    let keyboard = Keyboard::new_first_match("Tenkeyless-event-kbd", true).unwrap();

    loop {
        if let Ok(e) = keyboard.read_event() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
