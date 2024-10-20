// mouse events
use input_linux_tools::keyboard::*;

fn main() {
    let keyboard = Keyboard::new_first_match("Tenkeyless-event-kbd");

    loop {
        if let Ok(e) = keyboard.read_event() {
            println!("e {:?}", e);
        }
    }
}
