// keyboard events
use input_linux_tools::keyboard::*;
use std::{thread, time};

fn main() {
    let mut keyboard = Keyboard::new_first_match("Tenkeyless-event-kbd", false).unwrap();
    keyboard.ignore_autorepeat = true;

    loop {
        if let Some(e) = keyboard.read() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
