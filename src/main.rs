extern crate launchpad;
extern crate clap;
extern crate portmidi as pm;

use launchpad::*;

use std::thread;
use std::process;
use std::time::Duration;

mod cli;

fn main() {
    // initialize the PortMidi context.

    let args = cli::build_args();
    let inpt = args.get_matches();
    if inpt.occurrences_of("list") > 0 {
        list();
    }

    run();
}

fn list() -> ! {
    let context = pm::PortMidi::new().unwrap();
    let devs = context.devices().unwrap();

    for d in devs {
        println!("{:?}", d);
    }

    process::exit(0);
}

fn run() {
    println!("Please enjoy!");
    let timeout = Duration::from_millis(1);
    let mut lpad = LaunchpadPro::guess();

    println!("Clear screen...");
    lpad.light_all(0);

    // println!("Fuzzy!");
    // for r in 0..255 {
    //     if r % 10 != 0 {
    //         continue;
    //     }
    //     for g in 0..255 {
    //         if g % 10 != 0 {
    //             continue;
    //         }
    //         for b in 0..255 {
    //             if b % 10 != 0 {
    //                 continue;
    //             }
    //             lpad.light_fuzzy_rgb(11, r, g, b);
    //         }
    //     }
    // }
    // thread::sleep(Duration::from_millis(500));
    // println!("Fuzzy!");

    println!("Columns on!");
    for i in 0..10 {
        lpad.light_column(&ColorColumn {
            column: i,
            color: 5,
        });
        thread::sleep(Duration::from_millis(25));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Columns off!");
    for i in 0..10 {
        lpad.light_row(&ColorRow { row: i, color: 0 });
        thread::sleep(Duration::from_millis(25));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Whole panel colors...");
    for color in vec![18, 54, 13, 104, 0] {
        lpad.light_all(color);
        thread::sleep(Duration::from_millis(1000));
    }

    // Playground

    println!("Light rows in a silly way");
    for row in 1..9 {
        for column in 1..9 {
            let x = 10 * row + column;
            lpad.light_led(&ColorLed {
                position: x,
                color: 88,
            });
            thread::sleep(Duration::from_millis(1));
        }
        thread::sleep(Duration::from_millis(16));
    }

    thread::sleep(Duration::from_millis(500));

    println!("Bottom Right to Top Left");
    lpad.light_leds(&vec![&ColorLed {position: 11, color: 41,},
                          &ColorLed {position: 22, color: 41,},
                          &ColorLed {position: 33, color: 41,},
                          &ColorLed {position: 44, color: 41,},
                          &ColorLed {position: 55, color: 41,},
                          &ColorLed {position: 66, color: 41,},
                          &ColorLed {position: 77, color: 41,},
                          &ColorLed {position: 88, color: 41,},]);

    thread::sleep(Duration::from_millis(500));

    println!("Bottom Left to Top Right");
    lpad.light_leds(&vec![&ColorLed {position: 81, color: 5,},
                          &ColorLed {position: 72, color: 5,},
                          &ColorLed {position: 63, color: 5,},
                          &ColorLed {position: 54, color: 5,},
                          &ColorLed {position: 45, color: 5,},
                          &ColorLed {position: 36, color: 5,},
                          &ColorLed {position: 27, color: 5,},
                          &ColorLed {position: 18, color: 5,},]);

    thread::sleep(Duration::from_millis(500));

    println!("Right controls on");
    lpad.light_leds(&vec![&ColorLed {position: 19, color: 3,},
                          &ColorLed {position: 29, color: 3,},
                          &ColorLed {position: 39, color: 3,},
                          &ColorLed {position: 49, color: 3,},
                          &ColorLed {position: 59, color: 3,},
                          &ColorLed {position: 69, color: 3,},
                          &ColorLed {position: 79, color: 3,},
                          &ColorLed {position: 89, color: 3,},]);

    thread::sleep(Duration::from_millis(500));

    println!("Top controls on");
    lpad.light_leds(&vec![&ColorLed {position: 91, color: 4,},
                          &ColorLed {position: 92, color: 4,},
                          &ColorLed {position: 93, color: 4,},
                          &ColorLed {position: 94, color: 4,},
                          &ColorLed {position: 95, color: 4,},
                          &ColorLed {position: 96, color: 4,},
                          &ColorLed {position: 97, color: 4,},
                          &ColorLed {position: 98, color: 4,},]);

    thread::sleep(Duration::from_millis(500));

    println!("Left controls on");
    lpad.light_leds(&vec![&ColorLed {position: 10, color: 5,},
                          &ColorLed {position: 20, color: 5,},
                          &ColorLed {position: 30, color: 5,},
                          &ColorLed {position: 40, color: 5,},
                          &ColorLed {position: 50, color: 5,},
                          &ColorLed {position: 60, color: 5,},
                          &ColorLed {position: 70, color: 5,},
                          &ColorLed {position: 80, color: 5,},]);

    thread::sleep(Duration::from_millis(500));

    println!("Bottom controls on");
    lpad.light_leds(&vec![&ColorLed {position: 01, color: 6,},
                          &ColorLed {position: 02, color: 6,},
                          &ColorLed {position: 03, color: 6,},
                          &ColorLed {position: 04, color: 6,},
                          &ColorLed {position: 05, color: 6,},
                          &ColorLed {position: 06, color: 6,},
                          &ColorLed {position: 07, color: 6,},
                          &ColorLed {position: 08, color: 6,},]);

    thread::sleep(Duration::from_millis(500));
    println!("Blank screen");
    lpad.light_all(0);

    println!("Scroll Text");
    lpad.scroll_text(27, false, &format!("{}Your {}Turn!", SCROLL_SLOWER, SCROLL_FASTER));

    let mut foo = 0;

    println!("Blinky/Pulsy playground!");
    loop {
        if let Some(events) = lpad.poll() {
            // println!("{:?}", event);
            for press in events {
                if press.message.data2 == 127 {
                    foo += 1;
                    foo %= 128;
                    let led = ColorLed {
                        color: foo,
                        position: press.message.data1,
                    };
                    if 0x1 == (foo & 0x1) {
                        lpad.pulse_single(&led);
                    } else {
                        lpad.flash_single(&led);
                    }
                }
            }
        }

        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(timeout);
    }

}
