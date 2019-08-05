//! Definition of Launchpad devices.
//!
//! For now, only Launchpad Mark 2 devices are supported.

use portmidi::{InputPort, OutputPort, PortMidi};

use crate::color::nearest_palette;
use crate::RGBColor;

/// A Launchpad Mark 2 Device. This library requires the PortMidi device
/// used to create the launchpad to have the same lifetime. If we create the
/// PortMidi device ourselves, hold it. Otherwise, trust the implementer to
/// not destroy it (or further calls will fail (sometimes silently?))
pub struct LaunchpadMk2 {
    input_port: InputPort,
    output_port: OutputPort,
}

// TODO
#[derive(Debug)]
pub enum LaunchpadError {
    InvalidRawColor,
    InvalidPosition,
    Midi(portmidi::Error),
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

type Result<T> = std::result::Result<T, LaunchpadError>;

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

    // === RAW === //

    /// Light all LEDs to the same color
    pub fn light_all_raw(&self, raw_color: u8) -> Result<()> {
        // Message cannot be repeated.
        self.write_sysex(&[0x0E, raw_color])
    }

    /// Light a single LED to a color.
    pub fn light_single_raw(&self, x: u8, y: u8, color: u8) -> Result<()> {
        let pos = midi_from_coords((x, y))
            .ok_or(LaunchpadError::InvalidPosition)?;
        self.write_sysex(&[0x0A, pos, color])
    }

    /// Set a single LED to flash to a color.
    pub fn flash_single_raw(&self, x: u8, y: u8, color: u8) -> Result<()> {
        let pos = midi_from_coords((x, y))
            .ok_or(LaunchpadError::InvalidPosition)?;
        self.write_sysex(&[0x23, 0, pos, color])
    }

    /// Set a single LED to pulse to a color.
    pub fn pulse_single_raw(&self, x: u8, y: u8, color: u8) -> Result<()> {
        let pos = midi_from_coords((x, y))
            .ok_or(LaunchpadError::InvalidPosition)?;
        self.write_sysex(&[0x28, 0, pos, color])
    }

    /// Light a column of LEDs to the same color.
    pub fn light_column_raw(&self, x: u8, color: u8) -> Result<()> {
        if !is_valid_coord(&x) {
            return Err(LaunchpadError::InvalidPosition);
        }
        self.write_sysex(&[0x0C, x, color])
    }

    /// Light a row of LEDs to the same color.
    pub fn light_row_raw(&self, y: u8, color: u8) -> Result<()> {
        if !is_valid_coord(&y) {
            return Err(LaunchpadError::InvalidPosition);
        }
        self.write_sysex(&[0x0D, y, color])
    }

    /// Light a button LED to a color.
    pub fn light_button_raw(&self, coord: u8, top: bool, color: u8) -> Result<()> {
        let position = midi_from_button(coord, top)
            .ok_or(LaunchpadError::InvalidPosition)?;
        self.write_sysex(&[0x0A, position, color])
    }

    /// Begin scrolling a message. The screen will be blanked, and the letters
    /// will be the same color. If the message is set to loop, it can be cancelled
    /// by sending an empty `scroll_text` command. String should only contain ASCII
    /// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
    pub fn scroll_text_raw(&self, color: u8, do_loop: bool, text: &str) -> Result<()> {
        if !is_valid_color(&color) {
            return Err(LaunchpadError::InvalidRawColor)
        }

        let mut msg: Vec<u8> = vec![0x14, color, do_loop as u8];
        msg.extend_from_slice(text.as_bytes());

        self.write_sysex(&msg)
    }

    // === RGB === //

    /// Light all LEDs to the same RGB color.
    ///
    /// This is more expensive than `light_all_raw`.
    pub fn light_all(&self, color: &RGBColor) -> Result<()> {
        self.light_all_raw(nearest_palette(color))
    }

    /// Light a single LED to a RGB color.
    ///
    /// This is more expensive than `light_single_raw`.
    pub fn light_single(&self, x: u8, y: u8, color: &RGBColor) -> Result<()> {
        self.light_single_raw(x, y, nearest_palette(color))
    }

    /// Set a single LED to flash a RGB color.
    ///
    /// This is more expensive than `flash_single_raw`.
    pub fn flash_single(&self, x: u8, y: u8, color: &RGBColor) -> Result<()> {
        self.flash_single_raw(x, y, nearest_palette(color))
    }

    /// Set a single LED to pulse a RGB color.
    ///
    /// This is more expensive than `pulse_single_raw`.
    pub fn pulse_single(&self, x: u8, y: u8, color: &RGBColor) -> Result<()> {
        self.pulse_single_raw(x, y, nearest_palette(color))
    }

    /// Light a row of LEDs to the same RGB color.
    ///
    /// This is more expensive than `light_row_raw`.
    pub fn light_row(&self, y: u8, color: &RGBColor) -> Result<()> {
        self.light_row_raw(y, nearest_palette(color))
    }

    /// Light a column of LEDs to the same RGB color.
    ///
    /// This is more expensive than `light_column_raw`.
    pub fn light_column(&self, x: u8, color: &RGBColor) -> Result<()> {
        self.light_column_raw(x, nearest_palette(color))
    }

    /// Light a button LED to aRGB color.
    ///
    /// This is more expensive than `light_button_raw`
    pub fn light_button(&self, coord: u8, top: bool, color: &RGBColor) -> Result<()> {
        self.light_button_raw(coord, top, nearest_palette(color))
    }

    /// Begin scrolling a message. The screen will be blanked, and the letters
    /// will be the same RGB color. If the message is set to loop, it can be cancelled
    /// by sending an empty `scroll_text` command. String should only contain ASCII
    /// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
    ///
    /// This is more expensive than `scroll_text_raw`.
    pub fn scroll_text(&self, color: &RGBColor, do_loop: bool, text: &str) -> Result<()> {
        self.scroll_text_raw(nearest_palette(color), do_loop,  text)
    }

    /// Poll the device for MIDI events
    pub fn poll(&self) -> Result<Vec<Event>> {
        self.input_port.poll().or_else(|err| Err(LaunchpadError::Midi(err)))?;
        let events = self.input_port.read_n(1024).or_else(|err| Err(LaunchpadError::Midi(err)))?;
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

/// MIDI note value from Launchpad pad
fn midi_from_coords(coords: (u8, u8)) -> Option<u8> {
    if is_valid_coords(&coords) {
        Some(11 + coords.0 + coords.1 * 10)
    } else {
        None
    }
}

/// MIDI note value from Launchpad button
fn midi_from_button(coord: u8, top: bool) -> Option<u8> {
    if is_valid_coord(&coord) {
        if top {
            Some(104 + coord)
        } else {
            Some(10 * coord + 19)
        }
    } else {
        None
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
