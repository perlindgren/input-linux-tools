// list devices
use input_linux_tools::device::*;

fn main() {
    let devices = Devices::new().unwrap();

    println!("Keyboards");
    for k in &devices.keyboards {
        println!("{:?}", k);
    }

    println!("\nMice");
    for k in &devices.mice {
        println!("{:?}", k);
    }

    println!("\nGamepads");
    for k in devices.gamepads {
        println!("{:?}", k);
    }
}
