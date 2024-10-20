// raw linux events
use input_linux_tools::mouse::*;

fn main() {
    let mouse = Mouse::new_first_match("Pulsefire");

    loop {
        if let Ok(e) = mouse.read_event() {
            println!("e {:?}", e);
        }
    }
}
