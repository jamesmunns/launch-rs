use launchpad::mk2::{LaunchpadMk2, Location, MidiLaunchpadMk2};
use launchpad::RGBColor;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

const OFF: RGBColor = RGBColor::new(0x00, 0x00, 0x00);

fn main() -> Result<(), Box<dyn Error>> {
    let mut lp = MidiLaunchpadMk2::autodetect().unwrap();

    //
    // Exercise Launchpad Methods
    //
    let colors = [
        RGBColor::new(0xFF, 0x00, 0x00),
        RGBColor::new(0x00, 0xFF, 0x00),
        RGBColor::new(0x00, 0x00, 0xFF),
        RGBColor::new(0xFF, 0xFF, 0xFF),
        RGBColor::new(0xFF, 0xA5, 0x20),
    ];
    let mut col_iter = colors.iter().cycle();

    // fn light_all(&mut self, raw_color: u8) -> Result<()>;
    println!("Light All!");
    for col in colors.iter() {
        lp.light_all(col.nearest_midi())?;
        sleep(Duration::from_millis(500));
    }
    lp.light_all(OFF.nearest_midi())?;

    // fn light_single(&mut self, position: &Location, raw_color: u8) -> Result<()>;
    println!("Light Single!");
    for x in 0..8 {
        for y in 0..8 {
            lp.light_single(
                &Location::Pad(x, y),
                col_iter.next().unwrap().nearest_midi(),
            )?;
            sleep(Duration::from_millis(50));
        }
    }
    for i in 0..8 {
        lp.light_single(
            &Location::Button(i, launchpad::mk2::ButtonSide::Top),
            col_iter.next().unwrap().nearest_midi(),
        )?;
        sleep(Duration::from_millis(50));
    }
    for i in 0..8 {
        lp.light_single(
            &Location::Button(i, launchpad::mk2::ButtonSide::Right),
            col_iter.next().unwrap().nearest_midi(),
        )?;
        sleep(Duration::from_millis(50));
    }
    sleep(Duration::from_millis(3000));
    lp.light_all(OFF.nearest_midi())?;

    // fn light_multi(&mut self, leds: Vec<(Location, u8)>) -> Result<()>;

    // fn flash_single(&mut self, position: &Location, raw_color: u8) -> Result<()>;
    println!("Flash Single!");
    for x in 0..8 {
        for y in 0..8 {
            lp.flash_single(
                &Location::Pad(x, y),
                col_iter.next().unwrap().nearest_midi(),
            )?;
            sleep(Duration::from_millis(50));
        }
    }
    for i in 0..8 {
        lp.flash_single(
            &Location::Button(i, launchpad::mk2::ButtonSide::Top),
            col_iter.next().unwrap().nearest_midi(),
        )?;
        sleep(Duration::from_millis(50));
    }
    for i in 0..8 {
        lp.flash_single(
            &Location::Button(i, launchpad::mk2::ButtonSide::Right),
            col_iter.next().unwrap().nearest_midi(),
        )?;
        sleep(Duration::from_millis(50));
    }
    sleep(Duration::from_millis(3000));
    lp.light_all(OFF.nearest_midi())?;

    // fn pulse_single(&mut self, position: &Location, raw_color: u8) -> Result<()>;
    println!("Pulse Single!");
    for x in 0..8 {
        for y in 0..8 {
            lp.pulse_single(
                &Location::Pad(x, y),
                col_iter.next().unwrap().nearest_midi(),
            )?;
            sleep(Duration::from_millis(50));
        }
    }
    for i in 0..8 {
        lp.pulse_single(
            &Location::Button(i, launchpad::mk2::ButtonSide::Top),
            col_iter.next().unwrap().nearest_midi(),
        )?;
        sleep(Duration::from_millis(50));
    }
    for i in 0..8 {
        lp.pulse_single(
            &Location::Button(i, launchpad::mk2::ButtonSide::Right),
            col_iter.next().unwrap().nearest_midi(),
        )?;
        sleep(Duration::from_millis(50));
    }
    sleep(Duration::from_millis(3000));
    lp.light_all(OFF.nearest_midi())?;

    // fn light_row(&mut self, y: u8, raw_color: u8) -> Result<()>;
    println!("Light Row!");
    for row in 0..9 {
        lp.light_row(row, col_iter.next().unwrap().nearest_midi())?;
        sleep(Duration::from_millis(500));
    }
    sleep(Duration::from_millis(3000));
    lp.light_all(OFF.nearest_midi())?;

    // fn light_column(&mut self, x: u8, raw_color: u8) -> Result<()>;
    println!("Light Column!");
    for col in 0..9 {
        lp.light_column(col, col_iter.next().unwrap().nearest_midi())?;
        sleep(Duration::from_millis(500));
    }
    sleep(Duration::from_millis(3000));
    lp.light_all(OFF.nearest_midi())?;

    // fn light_single_rgb(&mut self, position: &Location, color: &RGBColor) -> Result<()>;
    // fn light_multi_rgb(&mut self, leds: Vec<(Location, RGBColor)>) -> Result<()>;
    // fn scroll_text(&mut self, do_loop: bool, text: &str, raw_color: u8) -> Result<()>;
    println!("Scroll Text!");
    for col in colors.iter().take(4) {
        lp.scroll_text(false, "Hello demo!", col.nearest_midi())?;
        sleep(Duration::from_millis(6000));
    }
    lp.light_all(OFF.nearest_midi())?;

    // fn poll(&mut self) -> Vec<Event>;
    // fn select_layout(&mut self, layout: Layout) -> Result<()>;
    // fn setup_fader(&mut self, index: u8, fader_type: FaderType, raw_color: u8, init_value: u8) -> Result<()>;

    Ok(())
}
