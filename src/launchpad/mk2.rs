//! Definition of Launchpad devices.
//!
//! For now, only Launchpad Mark 2 devices are supported.

use portmidi::{InputPort, OutputPort, PortMidi};

use crate::{nearest_palette, RGBColor, Launchpad};

use super::Result;

/// A Launchpad Mark 2 Device. This library requires the PortMidi device
/// used to create the launchpad to have the same lifetime. If we create the
/// PortMidi device ourselves, hold it. Otherwise, trust the implementer to
/// not destroy it (or further calls will fail (sometimes silently?))
pub struct LaunchpadMk2 {
    input_port: InputPort,
    output_port: OutputPort,
}

pub mod event {
    #[derive(Debug)]
    pub enum Event {
        Press(Location),
        Release(Location),
    }

    #[derive(Debug)]
    pub enum Location {
        Button(u8, bool),
        Pad(u8, u8),
    }
}

use event::*;

pub const SCROLL_SLOWEST: &'static str = "\u{01}";
pub const SCROLL_SLOWER: &'static str = "\u{02}";
pub const SCROLL_SLOW: &'static str = "\u{03}";
pub const SCROLL_NORMAL: &'static str = "\u{04}";
pub const SCROLL_FAST: &'static str = "\u{05}";
pub const SCROLL_FASTER: &'static str = "\u{06}";
pub const SCROLL_FASTEST: &'static str = "\u{07}";

impl LaunchpadMk2 {
    /// Attempt to find the first Launchpad Mark 2 by scanning
    /// available MIDI ports with matching names
    pub fn autodetect() -> Result<LaunchpadMk2> {
        let midi = PortMidi::new()
            .map_err(|err| LaunchpadError::Midi(err))?;
        Ok(Self::autodetect_from(&midi))?
    }

    /// Attempt to find the first Launchpad Mark 2 by scanning
    /// available MIDI ports with matching names. Bring your own
    /// PortMidi.
    pub fn autodetect_from(midi: &PortMidi) -> Result<LaunchpadMk2> {
        let devices = midi.devices()
            .map_err(|err| LaunchpadError::Midi(err))?;

        let mut input: Option<i32> = None;
        let mut output: Option<i32> = None;

        for d in devices {
            if !d.name().contains("Launchpad MK2") {
                continue;
            }

            if input.is_none() && d.is_input() {
                input = Some(d.id() as i32);
            }

            if output.is_none() && d.is_output() {
                output = Some(d.id() as i32);
            }

            // TODO use more idiomatic control flow?
            if input.is_some() && output.is_some() {
                break;
            }
        }

        let input_port = input.expect("No Launchpad MK2 input found!");
        let output_port = output.expect("No Launchpad MK2 output found!");

        let input_device = midi.device(input_port)
            .map_err(|err| LaunchpadError::Midi(err))?;
        let output_device = midi.device(output_port)
            .map_err(|err| LaunchpadError::Midi(err))?;

        let input = midi.input_port(input_device, 1024)
            .map_err(|err| LaunchpadError::Midi(err))?;
        let output = midi.output_port(output_device, 1024)
            .map_err(|err| LaunchpadError::Midi(err))?;

        Ok(LaunchpadMk2 {
            input_port: input,
            output_port: output,
        })
    }

    /// Poll the device for MIDI events
    pub fn poll(&self) -> Result<Vec<Event>> {
        self.input_port.poll().or_else(|err| Err(LaunchpadError::Midi(err)))?;
        let events = self.input_port.read_n(1024).or_else(|err| Err(err))?;
        Ok(events.map(|events| {
            events.iter().filter_map(|evt| {
                let value = evt.message.data1; // note

                // not magic
                let location = match value {
                    19 | 29 | 39 | 49 | 59 | 69 | 79 | 89 => Some(Location::Button(value / 10 - 1, false)), // side
                    11..=88 => Some(Location::Pad(value % 10 - 1, value / 10 - 1)),
                    104..=111 => Some(Location::Button(value - 104, true)), // top
                    _ => None,
                };

                location.map(|loc| {
                    if evt.message.data2 == 0 { // data2 => velocity
                        Event::Release(loc)
                    } else {
                        Event::Press(loc)
                    }
                }) // if location was none, this will be quietly filtered out
            }).collect()
        }).unwrap_or_default())
    }

    /// Write a SysEx message with the Launchpad Mk2 header
    fn write_sysex(&self, data: &[u8]) -> Result<()> {
        let mut msg = vec![0xF0, 0x00, 0x20, 0x29, 0x02, 0x18]; // header
        msg.extend_from_slice(data);
        msg.push(0xF7); // terminate

        self.output_port.write_sysex(0, &msg)
            .map_err(|e| LaunchpadError::Midi(e))
    }
}

impl Launchpad for LaunchpadMk2 {
    /// Light all LEDs to the same color
    fn light_all(&self, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0E, raw_color])
    }

    /// Light a single LED to a color.
    fn light_single(&self, raw_position: u8, color: u8) -> Result<()> {
        self.write_sysex(&[0x0A, pos, color])
    }

    /// Set a single LED to flash to a color.
    fn flash_single(&self, raw_position: u8, color: u8) -> Result<()> {
        self.write_sysex(&[0x23, 0, pos, color])
    }

    /// Set a single LED to pulse to a color.
    fn pulse_single(&self, raw_position: u8, color: u8) -> Result<()> {
        self.write_sysex(&[0x28, 0, pos, color])
    }

    /// Light a row of LEDs to the same color.
    fn light_row(&self, y: u8, color: u8) -> Result<()> {
        self.write_sysex(&[0x0D, y, color])
    }

    /// Light a column of LEDs to the same color.
    fn light_column(&self, x: u8, color: u8) -> Result<()> {
        self.write_sysex(&[0x0C, x, color])
    }

    /// Begin scrolling a message. The screen will be blanked, and the letters
    /// will be the same color. If the message is set to loop, it can be cancelled
    /// by sending an empty `scroll_text` command. String should only contain ASCII
    /// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
    fn scroll_text(&self, color: u8, do_loop: bool, text: &str) -> Result<()> {
        let mut msg: Vec<u8> = vec![0x14, color, do_loop as u8];
        msg.extend_from_slice(text.as_bytes());

        self.write_sysex(&msg)
    }
}

/// MIDI note value from Launchpad pad
pub fn pad_position(coords: (u8, u8)) -> u8 {
    assert!(is_valid_coords(&coords));

    11 + coords.0 + coords.1 * 10
}

/// MIDI note value from Launchpad button
pub fn button_position(coord: u8, top: bool) -> u8 {
    assert!(is_valid_coord(&coord));

    if top {
        104 + coord
    } else {
        10 * coord + 19
    }
}

fn is_valid_color(raw_color: &u8) -> bool {
    raw_color < &128
}

fn is_valid_coord(pos: &u8) -> bool {
    pos < &8
}

fn is_valid_coords(coords: &(u8, u8)) -> bool {
    is_valid_coord(&coords.0) && is_valid_coord(&coords.1)
}

//////////////////////////////////////////////////////////////////
// TODO ITEMS
//////////////////////////////////////////////////////////////////


// pub fn device_inquiry() {
//     // (240,126,127, 6, 1, 247)
// }

// #[derive(Debug)]
// enum Layout {
//     Session,
//     User_1,
//     User_2,
//     Ableton_Reserved,
//     Volume,
//     Pan
// }

// // pg6
// pub fn set_layout(layout: Layout) -> Result<()> {
//     use Layout::*;
//     let i = match layout {
//         Session => 0u8,
//         User_1 => 1u8,
//         User_2 => 2u8,
//         Ableton_Reserved => 3u8,
//         Volume => 4u8,
//         Pan => 5u8,
//     };
//     unimplemented!()
// }


// pub fn flash_led(led: &ColorLed) {
//     // F0h 00h 20h 29h 02h 18h 23h <LED> <Colour> F7h
//     // Message can be repeated up to 80 times.
//     flash_leds(&[led])
// }

// pub fn flash_leds(leds: &[&ColorLed]) {

// }

// pub fn pulse_led(led: &ColorLed) {
//     // F0h 00h 20h 29h 02h 18h 28h <LED> <Colour> F7h
//     // Message can be repeated up to 80 times.
//     pulse_leds(&[led])
// }

// pub fn pulse_leds(leds: &[&ColorLed]) {

// }

// pub fn light_rgb(light: u8, red: u8, green: u8, blue: u8) {
//     // F0h 00h 20h 29h 02h 18h 0Bh <LED>, <Red> <Green> <Blue> F7h
//     // Message can be repeated up to 80 times.
// }

// pub fn start_vol_fader() {

// }

// pub fn start_pan_fader() {

// }

// pub fn start_fader(layout: u8, number: u8, color: Color, value: u8)
// {

// }

// pub fn scroll_text(text: &[u8], loop: bool, color: Color) {

// }
