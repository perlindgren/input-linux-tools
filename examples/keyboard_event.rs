// mouse events
use input_linux_tools::keyboard::*;

fn main() {
    let mut keyboard = Keyboard::new_first_match("Tenkeyless-event-kbd");
    keyboard.ignore_autorepeat = true;

    loop {
        if let Some(e) = keyboard.read() {
            println!("e {:?}", e);
        }
    }
}
