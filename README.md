# adbridger

`adbridger` is a comprehensive Rust library for seamless integration and utilization of Android Debug Bridge (ADB). It simplifies the interaction with Android devices by allowing Rust applications to execute and manage ADB commands directly.

## Features

- **Complete ADB Integration**: Execute all standard ADB commands within Rust.
- **Ease of Use**: Simplified interface for interacting with Android devices.
- **Rust-Centric**: Designed specifically for Rust applications, ensuring compatibility and efficiency.

## Getting Started

### Prerequisites

- Rust programming environment
- Basic understanding of ADB

### Installation

Add `adbridger` to your Rust project's `Cargo.toml`:

```toml
[dependencies]
adbridger = {git = "https://github.com/suleymanlaarabi/adbridger-rs"}
```

### Usage

Here's a quick example to get you started:

- check device battery level

```
use adbridger::device;

fn main() {
    let devices =
        device::list_devices().expect("Impossible de recuperer la lise des appareille connecter");

    let first_device_battery_info = device::get_battery_info(&devices[0])
        .expect("Impossible de recuperer la batterie de l'apareille");

    println!("{}", first_device_battery_info.battery_level);
}


```

- display the screen of the device

```
use adbridger::{device, screen};

fn main() {
    let devices =
        device::list_devices().expect("Impossible de recuperer la lise des appareille connecter");

    screen::display_screen(&devices[0]).expect("Impossible denregistrer lecran de lapareille");
}

```

### Documentation

Comming soon.

### Contributing

Contributions to adbridger are welcome! Please read our contributing guidelines for more information.

### License

adbridger is released under the MIT License.
