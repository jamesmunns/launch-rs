//! Definition of Launchpad devices.
//!
//! For now, only Launchpad Mark 2 devices are supported.

use pm;
use color::nearest_palette;
use font::FONT8X8;
use ref_slice::ref_slice;

pub type Color = u8;

/// A Launchpad Mark 2 Device. This library requires the PortMidi device
/// used to create the launchpad to have the same lifetime. If we create the
/// PortMidi device ourselves, hold it. Otherwise, trust the implementer to
/// not destroy it (or further calls will fail (sometimes silently?))
pub struct LaunchpadMk2 {
    input_port: pm::InputPort,
    output_port: pm::OutputPort,
    midi: Option<pm::PortMidi>,
}

/// A single button/led
#[derive(Debug)]
pub struct ColorLed {
    pub color: Color,
    pub position: u8,
}

#[derive(Debug)]
/// A single column (0...8)
pub struct ColorColumn {
    pub color: Color,
    pub column: u8,
}

/// A single row (0...8)
#[derive(Debug)]
pub struct ColorRow {
    pub color: Color,
    pub row: u8,
}

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
    pub fn guess() -> LaunchpadMk2 {
        let midi = pm::PortMidi::new().expect("Failed to open PortMidi Instance!");
        let mut retval = Self::guess_from(&midi);
        retval.midi = Some(midi);
        retval
    }

    /// Attempt to find the first Launchpad Mark 2 by scanning
    /// available MIDI ports with matching names. Bring your own
    /// PortMidi.
    pub fn guess_from(midi: &pm::PortMidi) -> LaunchpadMk2 {
        let devs = midi.devices().expect("Failed to get Midi Device!");

        let mut input: Option<i32> = None;
        let mut output: Option<i32> = None;

        for d in devs {
            if !d.name().contains("Launchpad MK2") {
                continue;
            }

            if input.is_none() && d.is_input() {
                input = Some(d.id() as i32);
            }

            if output.is_none() && d.is_output() {
                output = Some(d.id() as i32);
            }

            if input.is_some() && output.is_some() {
                break;
            }
        }

        let input_port = input.expect("No Launchpad Mk2 Input Found!");
        let output_port = output.expect("No Launchpad Mk2 Output Found!");

        let input_device = midi.device(input_port)
            .expect("No Launchpad Input found!");
        let output_device = midi.device(output_port)
            .expect("No Launchpad Output found!");

        let input = midi.input_port(input_device, 1024)
            .expect("Failed to open port");
        let output = midi.output_port(output_device, 1024)
            .expect("Failed to open port");

        LaunchpadMk2 {
            input_port: input,
            output_port: output,
            midi: None,
        }
    }

    /// Set all LEDs to the same color
    pub fn light_all(&mut self, color: Color) {
        assert_color(color);
        // F0h 00h 20h 29h 02h 18h 0Eh <Colour> F7h
        // Message cannot be repeated.
        self.output_port
            .write_sysex(0, &[0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x0E, color, 0xF7])
            .expect("Fail");
    }

    /// Set a single LED to flash. Uses a smaller header than `flash_led` or
    /// `flash_leds` with a single item
    pub fn flash_single(&mut self, led: &ColorLed) {
        // ch2
        // (0x91, <btn>, <color>)
        assert_position(led.position);
        assert_color(led.color);
        self.output_port.write_message([0x91, led.position, led.color]).expect("Fail");
    }

    /// Set a single LED to pulse. Uses a smaller header than `pulse_led` or
    /// `pulse_leds` with a single item
    pub fn pulse_single(&mut self, led: &ColorLed) {
        // ch3
        // (0x92, <btn>, <color>)

        self.output_port.write_message([0x92, led.position, led.color]).expect("Fail");
    }

    /// Set a single LED to a palette color. Use `light_single` instead, its faster.
    pub fn light_led(&mut self, led: &ColorLed) {
        // F0h 00h 20h 29h 02h 18h 0Ah <LED> <Colour> F7h
        // Message can be repeated up to 80 times.

        self.light_leds(ref_slice(led));
    }

    /// Set LEDs to a certain color. Up to 80 LEDs can be set uniquely at once.
    pub fn light_leds(&mut self, leds: &[ColorLed]) {
        assert!(leds.len() <= 80);

        let mut msg: Vec<u8> = vec![0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x0A];
        for led in leds {
            assert_position(led.position);
            assert_color(led.color);
            msg.push(led.position);
            msg.push(led.color);
        }
        msg.push(0xF7);

        self.output_port.write_sysex(0, &msg)
            .expect("Fail");
    }

    /// Set the LEDs in a shape of a character
    pub fn light_char(&mut self, ch: char, pos: i8, color: Color) {
        assert!(pos < 8);
        assert!(pos > -8);
        assert_eq!(ch.len_utf8(), 1);

        let loc = ch as usize;

        let mut disp: Vec<ColorLed> = Vec::new();

        for row in (1..9).rev() {

            let mut char_color: Color;
            let row_char = FONT8X8[loc * 8 + (8 - row) as usize];

            for col in 1..9 {
                let position = row * 10 + col + pos;

                 if ((row_char << (col - 1)) & 0x80) == 0 || //check against font bitmap
                    position > (row * 10 + 8) || // out of bounds
                    position < (row * 10) { // out of bounds
                    char_color = 0;
                } else {
                    char_color = color;
                }

                if check_char_position(position as u8) {
                    disp.push(
                        ColorLed {
                            position: position as u8,
                            color: char_color,
                        }
                    );
                }
            }
        }
        self.light_leds(&disp);
    }

    /// Light a column of LEDs to the same color.
    pub fn light_column(&mut self, col: &ColorColumn) {
        // F0h 00h 20h 29h 02h 18h 0Ch <Column> <Colour> F7h
        // Message can be repeated up to 9 times.

        self.light_columns(ref_slice(col));
    }

    /// Light columns of LEDs to the same color. Each column may be set to a
    /// unique color. Up to 9 columns may be set at once.
    pub fn light_columns(&mut self, cols: &[ColorColumn]) {
        assert!(cols.len() <= 9);

        let mut msg: Vec<u8> = vec![0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x0C];
        for col in cols {
            assert_column(col.column);
            assert_color(col.color);
            msg.push(col.column);
            msg.push(col.color);
        }
        msg.push(0xF7);

        self.output_port.write_sysex(0, &msg)
            .expect("Fail");
    }

    /// Light a row of LEDs to the same color.
    pub fn light_row(&mut self, row: &ColorRow) {
        // F0h 00h 20h 29h 02h 18h 0Dh <Row> <Colour> F7h
        // Message can be repeated up to 9 times.
        self.light_rows(ref_slice(row));
    }

    /// Light rows of LEDs to the same color. Each row may be set to a
    /// unique color. Up to 9 rows may be set at once.
    pub fn light_rows(&mut self, rows: &[ColorRow]) {
        assert!(rows.len() <= 9);

        let mut msg: Vec<u8> = vec![0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x0D];
        for row in rows {
            assert_row(row.row);
            assert_color(row.color);
            msg.push(row.row);
            msg.push(row.color);
        }
        msg.push(0xF7);

        self.output_port.write_sysex(0, &msg)
            .expect("Fail");
    }

    /// Begin scrolling a message. The screen will be blanked, and the letters
    /// will be the same color. If the message is set to loop, it can be cancelled
    /// by sending an empty `scroll_text` command. String should only contain ASCII
    /// characters, or the byte value of 1-7 to set the speed (`\u{01}` to `\u{07}`)
    pub fn scroll_text(&mut self, color: Color, doloop: bool, text: &str) {
        // 14H <Color> <loop> <text...> F7h
        // Message cannot be repeated.
        assert_color(color);
        let mut msg: Vec<u8> =
            vec![0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x14, color, if doloop { 0x01 } else { 0x00 }];
        msg.extend_from_slice(text.as_bytes());
        msg.push(0xF7);

        self.output_port.write_sysex(0, &msg).expect("Fail");
    }

    /// Experimental. Try to set an LED by the color value in a "fast" way by
    /// by choosing the nearest neighbor palette color. This is faster because
    /// setting an LED using palette colors is a 3 byte message, whereas setting
    /// a specific RGB color takes at least 12 bytes.
    pub fn light_fuzzy_rgb(&mut self, position: u8, red: u8, green: u8, blue: u8) {
        self.light_led(&ColorLed {
            position: position,
            color: nearest_palette(red, green, blue),
        })
    }

    /// Retrieve pending MidiEvents
    pub fn poll(&self) -> Option<Vec<pm::MidiEvent>> {
        self.input_port.poll().expect("Closed Stream");
        self.input_port.read_n(1024).expect("Failed to read")
    }
}

fn check_char_position(pos: u8) -> bool{
    match pos {
        11...19 => true,
        21...29 => true,
        31...39 => true,
        41...49 => true,
        51...59 => true,
        61...69 => true,
        71...79 => true,
        81...89 => true,
        _ => false,
    }
}

/// Make sure the position is valid
fn assert_position(pos: u8) {
    // Probably just make a Result
    if !match pos {
        11...19 => true,
        21...29 => true,
        31...39 => true,
        41...49 => true,
        51...59 => true,
        61...69 => true,
        71...79 => true,
        81...89 => true,
        104...111 => true,
        _ => false,
    } {
        panic!("Bad Positon!")
    }
}

/// Make sure the palette color is valid
fn assert_color(clr: u8) {
    if clr > 127 {
        panic!("Bad Color!");
    }
}

/// Make sure the column is valid
fn assert_column(col: u8) {
    if col > 8 {
        panic!("Bad Column");
    }
}

/// Make sure the row is valid
fn assert_row(row: u8) {
    if row > 8 {
        panic!("Bad Row");
    }
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
