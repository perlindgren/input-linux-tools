// mouse events
use input_linux_tools::mouse::*;

fn main() {
    let mouse = Mouse::new_first_match("Pulsefire");

    loop {
        if let Some(e) = mouse.read() {
            println!("e {:?}", e);
        }
    }
}
