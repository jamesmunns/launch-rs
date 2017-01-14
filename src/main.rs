extern crate portmidi as pm;
extern crate clap;


use std::time::Duration;
use std::thread;
use std::process;

mod cli;
mod color;
mod cmds;

use cmds::*;

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

fn run() {
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(1);

    let ports = get_first_launchpad(&context);

    // get the device info for the given id
    let info = context.device(ports.input).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    // get the device's input port
    let in_port = context.input_port(info, 1024).unwrap();

    let info = context.device(ports.output).unwrap();
    let mut out_port = context.output_port(info, 1024).unwrap();

    for i in 0..9 {
        println!("SetupW: {:?}",
        out_port.write_sysex(0, &[240, 0, 32, 41, 2, 24, 12, i, 72, 247]));
        thread::sleep(Duration::from_millis(25));
    }

    for i in 0..9 {
        println!("SetupW: {:?}",
        out_port.write_sysex(0, &[240, 0, 32, 41, 2, 24, 12, i, 0, 247]));
        thread::sleep(Duration::from_millis(25));
    }

    println!("Ready!");

    // Playground

    for row in 1..9 {
        for column in 1..9 {
            let x = 10*row + column;


            light_led(&mut out_port, &ColorLed{ position: x, color: 88});


            thread::sleep(Duration::from_millis(1));
        }
        thread::sleep(Duration::from_millis(16));
    }

    thread::sleep(Duration::from_millis(500));

    light_leds(&mut out_port, &vec!(
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

    light_leds(&mut out_port, &vec!(
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

    light_leds(&mut out_port, &vec!(
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

    light_leds(&mut out_port, &vec!(
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



    // for i in 0..9 {
    //     println!("SetupW: {:?}",
    //     out_port.write_sysex(0, &[240, 0, 32, 41, 2, 24, 12, i, 0, 247]));
    //     thread::sleep(Duration::from_millis(25));
    // }

    // Wait for commands to complete
    thread::sleep(Duration::from_millis(1000));

}



// LISTEN AND COLOR

    // while let Ok(_) = in_port.poll() {
    //     if let Ok(Some(event)) = in_port.read_n(1024) {
    //         // println!("{:?}", event);
    //         for press in event {
    //             if press.message.data2 == 127 {
    //                 foo += 1;
    //                 foo %= 128;
    //                 println!("{} {:?}", foo, out_port.write_message([press.message.status, press.message.data1, foo]));
    //             }
    //         }
    //     }

    //     // foo += 5;
    //     // foo %= 60;

    //     // let val = out_port.write_sysex(0, &[0xf0, 240, 0, 32, 41, 2, 16, 11, 11, foo, 0, 0, 0xf7]);
    //     // println!("ColorW: {} {:?}", foo, val);

    //     // there is no blocking receive method in PortMidi, therefore
    //     // we have to sleep some time to prevent a busy-wait loop
    //     thread::sleep(timeout);
    // }
