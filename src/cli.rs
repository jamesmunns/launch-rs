use clap::{Arg, App};

pub fn build_args() -> App<'static, 'static> {
    App::new("launch-rs")
        .version("0.1")
        .author("James Munns <james.munns@gmail.com>")
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List available devices"))
}
