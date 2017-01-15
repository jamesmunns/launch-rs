# Rust Launchpad

A set of bindings for controlling a Novation Launchpad. Using PortMidi for Midi bindings.

Currently only supports the Launchpad MK2. If you have another Launchpad, please let me know [@bitshiftmask](https://twitter.com/bitshiftmask)!

Eventually, I would like to introduce a Launchpad Trait, so code can be generic across different Launchpad types. I accept pull requests!

* [Documentation](https://docs.rs/launchpad/0.1.0/launchpad/)
* [Crate](https://crates.io/crates/launchpad)

## Prerequisites

(excerpt from [portmidi-rs](https://github.com/musitdev/portmidi-rs))

You need to make sure you have the PortMidi library installed.

On Ubuntu / Debian:
```sh
apt-get install libportmidi-dev
```

Arch Linux:
```sh
pacman -S portmidi
```

On OSX (Homebrew):
```sh
brew install portmidi
```
On OSX, if you get a linker error `ld: library not found for -lportmidi`, either,
 - make sure you have the Xcode Command Line Tools installed, not just Xcode, or
 - make sure you have the PortMidi library in your `$LIBRARY_PATH`, e.g. for Homebrew:

   ```sh
   export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"
   ```

## Use

First, add `launchpad` to your Cargo.toml:

```toml
[dependencies]
launchpad = "0.1"
```

Then, get started!

```rust
extern crate launchpad;

use std::thread;
use launchpad::*;

fn main() {
    let mut lpad = LaunchpadMk2::guess();

    // Output
    println!("Clear screen...");
    lpad.light_all(0);

    println!("Columns on!");
    for i in 0..9 {
        lpad.light_column(&ColorColumn {column: i, color: 5});
        thread::sleep(Duration::from_millis(25));
    }

    thread::sleep(Duration::from_millis(500));
    lpad.light_all(0);

    // Input and Output
    loop {
        if let Some(events) = lpad.poll() {
            for press in events {
                if press.message.data2 == 127 {
                    lpad.pulse_single(&ColorLed {
                        color: foo,
                        position: press.message.data1,
                    });
                }
            }
        }

        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(timeout);
    }
}
```

## References
* [Palette Table Information](http://launchpaddr.com/mk2palette/)
* [Launchpad Mk2 Programmers Reference Manual](https://global.novationmusic.com/sites/default/files/novation/downloads/10529/launchpad-mk2-programmers-reference-guide_0.pdf) (PDF warning)

## License

This code is licensed under the MIT license.
