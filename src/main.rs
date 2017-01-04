extern crate portmidi as pm;
extern crate clap;


use std::time::Duration;
use std::thread;
use std::process;

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
    let devs = midi.devices().unwrap();

    let mut input: Option<i32> = None;
    let mut output: Option<i32> = None;

    for d in devs {
        match (input.is_none(), d.is_input()) {
            (true, true) => input = Some(d.id() as i32),
            (false, true) => {panic!()} // multiple inputs
            _ => {}
        }
        match (output.is_none(), d.is_output()) {
            (true, true) => output = Some(d.id() as i32),
            (false, true) => {panic!()} // multiple outputs
            _ => {}
        }
        if input.is_some() && output.is_some() {
            break;
        }
    };

    LaunchPorts{input: input.unwrap(), output: output.unwrap()}
}

// f0 [ 240, 0, 32, 41, 2, 16, 11, number, red, green, blue ] f7

    // def LedCtrlRawByCode( self, number, colorcode = None ):

    //     number = min( number, 111 )
    //     number = max( number, 0 )

    //     if number > 89 and number < 104:
    //         return

    //     # TODO: limit/check colorcode
    //     if colorcode is None:
    //         colorcode = LaunchpadPro.COLORS['white']

    //     if number < 104:
    //         self.midi.RawWrite( 144, number, colorcode )
    //     else:
    //         self.midi.RawWrite( 176, number, colorcode )

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
        println!("SetupW: {:?}", out_port.write_sysex(0, &[ 240, 0, 32, 41, 2, 24, 12, i, 72, 247 ]));
        thread::sleep(Duration::from_millis(100));
    }

    for i in 0..9 {
        println!("SetupW: {:?}", out_port.write_sysex(0, &[ 240, 0, 32, 41, 2, 24, 12, i, 0, 247 ]));
        thread::sleep(Duration::from_millis(100));
    }

    let _ = out_port.write_sysex(0, &[240, 0, 32, 41, 2, 4, 20, 124, 1, 5, 72, 101, 108, 108, 111, 32, 2, 119, 111, 114, 108, 100, 33, 247]);

    let mut foo = 0;


    thread::sleep(timeout*1000);


    // let _ = out_port.write_message([146, 88, 81]);

    while let Ok(_) = in_port.poll() {
        if let Ok(Some(event)) = in_port.read_n(1024) {
            // println!("{:?}", event);
            for press in event {
                if press.message.data2 == 127 {
                    foo += 1;
                    foo %= 128;
                    println!("{} {:?}", foo, out_port.write_message([press.message.status, press.message.data1, foo]));
                }
            }
        }

        // foo += 5;
        // foo %= 60;

        // let val = out_port.write_sysex(0, &[0xf0, 240, 0, 32, 41, 2, 16, 11, 11, foo, 0, 0, 0xf7]);
        // println!("ColorW: {} {:?}", foo, val);

        // there is no blocking receive method in PortMidi, therefore
        // we have to sleep some time to prevent a busy-wait loop
        thread::sleep(timeout);
    }
}
