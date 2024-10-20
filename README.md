# linux-input-tools

Input from mice and keyboards using `linux-input`, allowing to treat each input device seprately, e.g., your application can use several mice and keyboards for different purposes, e.g., a alocal multi-player game where each player has their own keyboard and mouse, or even using two mice to control one player. The library puts no limitations to usage. Input is captured before the compositor, thus input does not take notice of "window/focus", which is typically what you want when it cames to games or other applictions with "exclusive" behavior.

The behavior is essentially stateless, thus device state needs to be maintained by your applaction. Future work (might) include a statefull wrapper, similar to [Bevy](https://bevyengine.org/) input systems.

---

## Device detection

Devices are assumed to be presented under `dev/input/`. The `by-id` folder provides human readble identification for the connected devices. The library provides a `Device` abstraction, filtering out devices based on reported capabilities. Notice, some devices present themself as both mice, joysticks (gamepads etc.). Some devices like keyboards may present themselves as several keyboards.

```rust
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
```
Expected output, your milage might vary:
```shell
Keyboards
"/dev/input/by-id/usb-Kingston_Technology_Company_HyperX_Pulsefire_FPS_Pro-if01-event-kbd"
"/dev/input/by-id/usb-Contour_Design_RollerMouse_Re:d-if01-event-kbd"
"/dev/input/by-id/usb-Razer_Razer_BlackWidow_V3_Tenkeyless-if01-event-kbd"
"/dev/input/by-id/usb-Razer_Razer_BlackWidow_V3_Tenkeyless-event-kbd"

Mice
"/dev/input/by-id/usb-Contour_Design_RollerMouse_Re:d-event-mouse"
"/dev/input/by-id/usb-Kingston_Technology_Company_HyperX_Pulsefire_FPS_Pro-event-mouse"
"/dev/input/by-id/usb-Razer_Razer_BlackWidow_V3_Tenkeyless-if02-event-mouse"
"/dev/input/by-id/usb-Sony_Interactive_Entertainment_DualSense_Wireless_Controller-if03-event-mouse"

Gamepads
"/dev/input/by-id/usb-Sony_Interactive_Entertainment_DualSense_Wireless_Controller-if03-event-joystick"
```

---

## Mouse

You can listen to mouse events per device. In this example it matches my "Pulsefire" mouse from the above devices.

```rust
// mouse events
use input_linux_tools::mouse::*;
use std::{thread, time};
fn main() {
    let mouse = Mouse::new_first_match("Pulsefire", false).unwrap();

    loop {
        if let Some(e) = mouse.read() {
            println!("e {:?}", e);
        } else {
            println!("-- sleep --");
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
```

---

## Keyboard

You can also listen to keyboard events per device. In this case my Razer keyboard (see device listing above).

```rust
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
```

---

## Gamepads

While `evdev` supports gamepads (joysticks), however support is not yet implemenented. (The built in gamepad support of Bevy is quite good.)

---

## Future work

What's lacking:
- The mouse implementation currently does not support scroll wheel. This should be fairly straightforwarde.
- No abstraction on top of `evdev` keycode enumaration (which comprises > 700 key events). Adopting the Bevy key codes would likely be a better option.
- As mentioned, gamepad support is not there yet.

What could be done:
- Kernel level time-stamping. This could be beneficial to precise timing in context of online gaming (and other cases where timing is crucial).
- Cross platform support. The same api could potentially be shared among Linux, Windows and OSX platforms, providing per device event input cross relevant operating systems.

---

## License

`input-linux-tools` is free, open source and permissively licensed! All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
