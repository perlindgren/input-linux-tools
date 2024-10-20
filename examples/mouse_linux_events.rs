// mouse, linux events
use input_linux_tools::mouse::*;
use std::{thread, time};

fn main() {
    let mouse = Mouse::new_first_match("Pulsefire", false).unwrap();

    loop {
        if let Ok(e) = mouse.read_event() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
