#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

fn main() {
    clap::App::new("rchat")
        .version("0.1")
        .about("An experimental chat server.")
        .author("Wendell Smith")
        .get_matches();

    println!("Hello, world!");
}
