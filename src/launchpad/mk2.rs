//! Definition of Launchpad devices.
//!
//! For now, only Launchpad Mark 2 devices are supported.

use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection, SendError};

use std::sync::{Arc, Mutex};

// use portmidi::{InputPort, OutputPort, PortMidi};

use crate::Launchpad;

pub const DEVICE_NAME: &'static str = "Launchpad MK2";

/// A Launchpad Mark 2 Device. This library requires the PortMidi device
/// used to create the launchpad to have the same lifetime. If we create the
/// PortMidi device ourselves, hold it. Otherwise, trust the implementer to
/// not destroy it (or further calls will fail (sometimes silently?))
pub struct LaunchpadMk2 {
    midi_input: MidiInputConnection<Arc<Mutex<Vec<Mk2Event>>>>,
    midi_output: MidiOutputConnection,
    events: Arc<Mutex<Vec<Mk2Event>>>,
}

#[derive(Clone, Debug)]
pub enum Mk2Event {
    Press(Location),
    Release(Location),
}

#[derive(Clone, Debug)]
pub enum Location {
    Button(u8, bool),
    Pad(u8, u8),
}

pub const SCROLL_SLOWEST: &'static str = "\u{01}";
pub const SCROLL_SLOWER: &'static str = "\u{02}";
pub const SCROLL_SLOW: &'static str = "\u{03}";
pub const SCROLL_NORMAL: &'static str = "\u{04}";
pub const SCROLL_FAST: &'static str = "\u{05}";
pub const SCROLL_FASTER: &'static str = "\u{06}";
pub const SCROLL_FASTEST: &'static str = "\u{07}";

pub type Result<T> = std::result::Result<T, SendError>;

impl LaunchpadMk2 {
    /// Attempt to find the first Launchpad Mark 2 by scanning
    /// available MIDI ports with matching names
    pub fn autodetect() -> std::result::Result<LaunchpadMk2, midir::InitError> {
		let mut input = MidiInput::new(DEVICE_NAME)?;
        let output = MidiOutput::new(DEVICE_NAME)?;

        input.ignore(Ignore::None);

        let events = Arc::new(Mutex::new(Vec::new()));

        // TODO handle errors
        let input = input.connect(1, "launchpad-rs-in", |_timestamp, message, data| {
            if let Some(event) = parse_message(message) {
                let mut events = data.lock().unwrap();
                events.push(event);
            }
        }, events.clone()).unwrap();
        let output = output.connect(1, "launchpad-rs-out").unwrap();

        Ok(LaunchpadMk2 {
            midi_input: input,
            midi_output: output,
            events,
        })
    }

    /// Write a SysEx message with the Launchpad Mk2 header
    fn write_sysex(&mut self, data: &[u8]) -> Result<()> {
        let mut msg = vec![0xF0, 0x00, 0x20, 0x29, 0x02, 0x18]; // header
        msg.extend_from_slice(data);
        msg.push(0xF7); // terminate

        self.midi_output.send(&msg)
    }
}

impl Launchpad<Mk2Event> for LaunchpadMk2 {
    /// Light all LEDs to the same color
    fn light_all(&mut self, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0E, raw_color])
    }

    /// Light a single LED to a color.
    fn light_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0A, raw_position, raw_color])
    }

    /// Set a single LED to flash to a color.
    fn flash_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x23, 0, raw_position, raw_color])
    }

    /// Set a single LED to pulse to a color.
    fn pulse_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x28, 0, raw_position, raw_color])
    }

    /// Light a row of LEDs to the same color.
    fn light_row(&mut self, y: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0D, y, raw_color])
    }

    /// Light a column of LEDs to the same color.
    fn light_column(&mut self, x: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0C, x, raw_color])
    }

    fn light_single_rgb(&mut self, raw_position: u8, red: u8, green: u8, blue: u8) -> Result<()> {
        self.write_sysex(&[0x0B, raw_position, red, green, blue])
    }

    /// Begin scrolling a message. The screen will be blanked, and the letters
    /// will be the same color. If the message is set to loop, it can be cancelled
    /// by sending an empty `scroll_text` command. String should only contain ASCII
    /// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
    fn scroll_text(&mut self, do_loop: bool, text: &str, raw_color: u8) -> Result<()> {
        let mut msg: Vec<u8> = vec![0x14, raw_color, do_loop as u8];
        msg.extend_from_slice(text.as_bytes());

        self.write_sysex(&msg)
    }

    fn poll(&mut self) -> Vec<Mk2Event> {
        let mut events = self.events.lock().unwrap();
        let events_clone = events.clone();
        events.clear();
        events_clone
    }
}

fn parse_message(message: &[u8]) -> Option<Mk2Event> {
    if message.len() < 3 {
        return None;
    }

    let value = message[1];
    let velocity = message[2];

    // not magic
    let location = match value {
        19 | 29 | 39 | 49 | 59 | 69 | 79 | 89 => Some(Location::Button(value / 10 - 1, false)), // side
        11..=88 => Some(Location::Pad(value % 10 - 1, value / 10 - 1)),
        104..=111 => Some(Location::Button(value - 104, true)), // top
        _ => None,
    };

    location.map(|loc| {
        if velocity == 0 {
            Mk2Event::Release(loc)
        } else {
            Mk2Event::Press(loc)
        }
    })
}

/// MIDI note value from Launchpad pad
pub fn pad_position(x: u8, y: u8) -> u8 {
    assert!(is_valid_coord(&x) && is_valid_coord(&y));

    11 + x + y * 10
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
