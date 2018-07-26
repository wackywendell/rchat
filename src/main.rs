#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

fn main() {
    let server_subparser = clap::SubCommand::with_name("serve").arg(
        clap::Arg::with_name("port")
            .short("p")
            .long("port")
            .help("Set port to serve on"),
    );

    clap::App::new("rchat")
        .version("0.1")
        .about("An experimental chat server.")
        .author("Wendell Smith")
        .subcommand(server_subparser)
        .get_matches();

    println!("Hello, world!");
}
