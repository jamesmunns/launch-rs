use pm::{OutputPort};

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

pub type Color = u8;

#[derive(Debug)]
pub struct ColorLed {
    pub color: Color,
    pub position: u8,
}

fn assert_position(pos: u8) {
    // Probably just make a Result
    if !match pos {
        11 ...  19 => true,
        21 ...  29 => true,
        31 ...  39 => true,
        41 ...  49 => true,
        51 ...  59 => true,
        61 ...  69 => true,
        71 ...  79 => true,
        81 ...  89 => true,
        104 ... 111 => true,
        _ => false
    } {panic!("Bad Positon!")}

}

// struct ColorColumn {
//     color: Color,
//     column: u8,
// }

// struct ColorRow {
//     color: Color,
//     row: u8,
// }

pub fn flash(port: &mut OutputPort, led: &ColorLed) {
    // ch2
    // (0x91, <btn>, <color>)
    assert_position(led.position);
    port.write_message([0x91, led.position, led.color]).expect("Fail");
}

pub fn pulse(port: &mut OutputPort, led: &ColorLed) {
    // ch3
    // (0x92, <btn>, <color>)

    port.write_message([0x92, led.position, led.color]).expect("Fail");
}

// pub fn device_inquiry() {
//     // (240,126,127, 6, 1, 247)
// }

pub fn light_led(port: &mut OutputPort, led: &ColorLed) {
    // F0h 00h 20h 29h 02h 18h 0Ah <LED> <Colour> F7h
    // Message can be repeated up to 80 times.
    light_leds(port, &[led])
}

pub fn light_leds(port: &mut OutputPort, leds: &[&ColorLed]) {
    for led in leds {
        assert_position(led.position);
        port.write_sysex(0, &[0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x0A, led.position, led.color, 0xF7]).expect("Fail");
    }
}

// pub fn light_column(port: &mut OutputPort, col: &ColorColumn) {
//     // F0h 00h 20h 29h 02h 18h 0Ch <Column> <Colour> F7h
//     // Message can be repeated up to 9 times.
//     light_columns(port, &[col])
// }

// pub fn light_columns(port: &mut OutputPort, cols: &[&ColorColumn]) {
//     for led in leds {
//         println!("{:?}", led);
//         port.write_sysex(0, &[0xF0, 0x00, 0x20, 0x29, 0x02, 0x18, 0x0C, led.position, led.color, 0xF7]).expect("Fail");
//     }
// }

// pub fn light_row(row: &ColorRow) {
//     // F0h 00h 20h 29h 02h 18h 0Dh <Row> <Colour> F7h
//     // Message can be repeated up to 9 times.
//     light_rows(&[row])
// }

// pub fn light_rows(rows: &[&ColorRow]) {

// }

// pub fn light_all(color: Color) {
//     // F0h 00h 20h 29h 02h 18h 0Eh <Colour> F7h
//     // Message cannot be repeated.
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
