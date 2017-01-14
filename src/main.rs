extern crate portmidi as pm;
extern crate clap;


use std::time::Duration;
use std::thread;
use std::process;

mod cli;
mod color;
mod launchpad;

// use cmds::*;
use launchpad::*;

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

#[derive(Debug)]
struct LaunchPorts {
    input: i32,
    output: i32,
}

// fn get_first_launch(midi: &pm::PortMidi) -> Result<LaunchPorts, DumbError> {
//     let _ = midi.devices()?;
//     // for d in midi.devices()? {

//     // }
//     Err(DumbError{})
// }

fn get_first_launchpad(midi: &pm::PortMidi) -> LaunchPorts {
    let devs = midi.devices().expect("Failed to get Midi Device!");

    let mut input: Option<i32> = None;
    let mut output: Option<i32> = None;

    for d in devs {
        if d.name() != "Launchpad MK2" {
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

    LaunchPorts {
        input: input.expect("No Launchpad Input found!"),
        output: output.expect("No Launchpad Output found!"),
    }
}

use launchpad::*;

fn run() {
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(1);

    let mut lpad = LaunchpadMk2::guess(&context);

    lpad.light_all(0);

    for i in 0..9 {
        lpad.light_column(&ColorColumn {column: i, color: 5});
        thread::sleep(Duration::from_millis(25));
    }

    thread::sleep(Duration::from_millis(500));

    for i in 0..9 {
        lpad.light_row(&ColorRow {row: i, color: 0});
        thread::sleep(Duration::from_millis(25));
    }

    thread::sleep(Duration::from_millis(500));


    for color in vec!(18, 54, 13, 104, 0) {
        lpad.light_all(color);
        thread::sleep(Duration::from_millis(1000));
    }

    println!("Ready!");

    // Playground

    for row in 1..9 {
        for column in 1..9 {
            let x = 10*row + column;


            lpad.light_led(&ColorLed{ position: x, color: 88});


            thread::sleep(Duration::from_millis(1));
        }
        thread::sleep(Duration::from_millis(16));
    }

    thread::sleep(Duration::from_millis(500));

    lpad.light_leds(&vec!(
        &ColorLed{ position: 11, color: 41},
        &ColorLed{ position: 22, color: 41},
        &ColorLed{ position: 33, color: 41},
        &ColorLed{ position: 44, color: 41},
        &ColorLed{ position: 55, color: 41},
        &ColorLed{ position: 66, color: 41},
        &ColorLed{ position: 77, color: 41},
        &ColorLed{ position: 88, color: 41}
    ));

    thread::sleep(Duration::from_millis(500));

    lpad.light_leds(&vec!(
        &ColorLed{ position: 81, color: 5},
        &ColorLed{ position: 72, color: 5},
        &ColorLed{ position: 63, color: 5},
        &ColorLed{ position: 54, color: 5},
        &ColorLed{ position: 45, color: 5},
        &ColorLed{ position: 36, color: 5},
        &ColorLed{ position: 27, color: 5},
        &ColorLed{ position: 18, color: 5}
    ));

    thread::sleep(Duration::from_millis(500));

    lpad.light_leds(&vec!(
        &ColorLed{ position: 19, color: 3},
        &ColorLed{ position: 29, color: 3},
        &ColorLed{ position: 39, color: 3},
        &ColorLed{ position: 49, color: 3},
        &ColorLed{ position: 59, color: 3},
        &ColorLed{ position: 69, color: 3},
        &ColorLed{ position: 79, color: 3},
        &ColorLed{ position: 89, color: 3}
    ));

    thread::sleep(Duration::from_millis(500));

    lpad.light_leds(&vec!(
        &ColorLed{ position: 104, color: 4},
        &ColorLed{ position: 105, color: 4},
        &ColorLed{ position: 106, color: 4},
        &ColorLed{ position: 107, color: 4},
        &ColorLed{ position: 108, color: 4},
        &ColorLed{ position: 109, color: 4},
        &ColorLed{ position: 110, color: 4},
        &ColorLed{ position: 111, color: 4}
    ));

    // playground

    thread::sleep(Duration::from_millis(500));
    lpad.light_all(0);
    lpad.scroll_text(27, false, "Your Turn!");

    let mut foo = 0;

    loop {
        if let Some(events) = lpad.poll() {
            // println!("{:?}", event);
            for press in events {
                if press.message.data2 == 127 {
                    foo += 1;
                    foo %= 128;
                    // println!("{} {:?}", foo, out_port.write_message([press.message.status, press.message.data1, foo]));
                    lpad.pulse_single(&ColorLed{ color: foo, position: press.message.data1 })
                }
            }
        }

        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(timeout);
    }

    // Wait for commands to complete
    thread::sleep(Duration::from_millis(1000));

}
