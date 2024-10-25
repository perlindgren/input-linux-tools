// mouse events
use input_linux_tools::mouse::*;
use std::{thread, time};

fn main() {
    let mouse = Mouse::new_first_match("Mouse-event-mouse", true).unwrap();

    loop {
        if let Some(e) = mouse.read() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
