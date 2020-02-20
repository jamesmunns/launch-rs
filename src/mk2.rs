//! Definition of Launchpad devices.
//!
//! For now, only Launchpad Mark 2 devices are supported.

use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection, SendError};

use std::sync::{Arc, Mutex};

pub const DEVICE_NAME: &'static str = "Launchpad MK2";

pub trait LaunchpadMk2 {
	/// Light all LEDs to the same color.
	fn light_all(&mut self, color: u8) -> Result<()>;

	/// Light a single LED to a color.
	fn light_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Set a single LED to flash a color.
	fn flash_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Set a sinle LED to pulse a color.
	fn pulse_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()>;

	/// Light a row of LEDs to the same color, including the side buttons.
	fn light_row(&mut self, y: u8, raw_color: u8) -> Result<()>;

	/// Light a column of LEDs to the same color, including the side buttons.
	fn light_column(&mut self, x: u8, raw_color: u8) -> Result<()>;

	fn light_single_rgb(&mut self, raw_position: u8, red: u8, green: u8, blue: u8) -> Result<()>;

	/// Begin scrolling a message. The screen will be blanked, and the letters
	/// will be the same color. If the message is set to loop, it can be cancelled
	/// by sending an empty `scroll_text` command. String should only contain ASCII
	/// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
	fn scroll_text(&mut self, do_loop: bool, text: &str, raw_color: u8) -> Result<()>;

    /// Select the current [Layout]. See device documentation.
    fn select_layout(&mut self, layout: Layout) -> Result<()>;

    /// Setup the nth fader (index 0-7). Read values (0-127) changed from [poll].
    /// Faders will not be active unless the layout is also changed.
    fn setup_fader(&mut self, index: u8, fader_type: FaderType, raw_color: u8, init_value: u8) -> Result<()>;

	/// Poll the device for MIDI events.
	fn poll(&mut self) -> Vec<Mk2Event>;
}

/// A Launchpad Mk2 device connected through MIDI.
/// The connection will close on drop.
pub struct MidiLaunchpadMk2 {
    _midi_input: MidiInputConnection<Arc<Mutex<Vec<Mk2Event>>>>,
    midi_output: MidiOutputConnection,
    events: Arc<Mutex<Vec<Mk2Event>>>,
}

#[derive(Clone, Debug)]
pub enum Mk2Event {
    Press(Location),
    Release(Location),
    FaderUpdate(u8, u8),
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

impl MidiLaunchpadMk2 {
    /// Attempt to find the first Launchpad Mark 2 by scanning
    /// available MIDI ports with matching names
    pub fn autodetect() -> std::result::Result<MidiLaunchpadMk2, midir::InitError> {
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

        Ok(MidiLaunchpadMk2 {
            _midi_input: input,
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

impl LaunchpadMk2 for MidiLaunchpadMk2 {
    fn light_all(&mut self, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0E, raw_color])
    }

    fn light_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0A, raw_position, raw_color])
    }

    fn flash_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x23, 0, raw_position, raw_color])
    }

    fn pulse_single(&mut self, raw_position: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x28, 0, raw_position, raw_color])
    }

    fn light_row(&mut self, y: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0D, y, raw_color])
    }

    fn light_column(&mut self, x: u8, raw_color: u8) -> Result<()> {
        self.write_sysex(&[0x0C, x, raw_color])
    }

    fn light_single_rgb(&mut self, raw_position: u8, red: u8, green: u8, blue: u8) -> Result<()> {
        self.write_sysex(&[0x0B, raw_position, red, green, blue])
    }

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

    fn select_layout(&mut self, layout: Layout) -> Result<()> {
        self.write_sysex(&[0x22, layout.value()])
    }

    fn setup_fader(&mut self, index: u8, fader_type: FaderType, raw_color: u8, init_value: u8) -> Result<()> {
        self.write_sysex(&[0x2B, index, fader_type.value(), raw_color, init_value])
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

fn is_valid_coord(pos: &u8) -> bool {
    pos < &8
}


#[derive(Debug)]
pub enum Layout {
    Session,
    User1,
    User2,
    AbletonReserved,
    Volume,
    Pan
}

impl Layout {
    fn value(&self) -> u8 {
        use Layout::*;
        match self {
            Session => 0,
            User1 => 1,
            User2 => 2,
            AbletonReserved => 3,
            Volume => 4,
            Pan => 5,
        }
    }
}

#[derive(Debug)]
pub enum FaderType {
    Volume,
    Pan,
}

impl FaderType {
    fn value(&self) -> u8 {
        use FaderType::*;
        match self {
            Volume => 0,
            Pan => 1,
        }
    }
}

// TODO device inquiry, version inquiry, and set to bootloader?
