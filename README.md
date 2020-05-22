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

On Fedora:
```sh
dnf install portmidi-devel
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
launchpad = "2.0"
```

Then, get started!

TODO: add up-to-date example

## References
* [Palette Table Information](http://launchpaddr.com/mk2palette/)
* [Launchpad Mk2 Programmers Reference Manual](https://d2xhy469pqj8rc.cloudfront.net/sites/default/files/novation/downloads/10529/launchpad-mk2-programmers-reference-guide-v1-02.pdf) (PDF warning)

## License

This code is licensed under the MIT license.