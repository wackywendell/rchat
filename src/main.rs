#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use protos::chat_grpc;

use futures::{Future, Stream};
use structopt::StructOpt;

pub mod client;
pub mod server;

// See https://gist.github.com/rust-play/0a90015498aaad3b1f8321364a1ff035

#[derive(StructOpt, Debug)]
struct ServeOpt {
    #[structopt(short = "p", long = "port", default_value = "6789")]
    port: u16,
}

#[derive(StructOpt, Debug)]
struct ClientOpt {
    #[structopt(short = "p", long = "port", default_value = "6789")]
    port: u16,

    #[structopt(short = "n", long = "name", default_value = "anon")]
    name: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rchat", about = "A simple chat client and server")]
enum Opt {
    #[structopt(name = "serve")]
    Serve(ServeOpt),

    #[structopt(name = "client")]
    Client(ClientOpt),
}

fn client(o: &ClientOpt) -> Result<(), grpc::Error> {
    let client = chat_grpc::ChatClient::new_plain("::1", o.port, Default::default())?;
    let cli = client::ChatClient::register(client, o.name.clone())?;

    let listener = cli.listen();
    std::thread::spawn(move || {
        let r = listener
            .for_each(|m| {
                println!("{}: {}", m.name, m.message);
                Ok(())
            }).wait();
        match r {
            Ok(()) => {}
            Err(e) => {
                println!("Error listening: {}", e);
            }
        }
    });

    let mut input = String::new();

    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                cli.say(input.clone())?;
                input.clear();
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }

    return Ok(());
}

fn serve(s: &ServeOpt) -> Result<(), grpc::Error> {
    let handler = server::ChatServer::new();
    let mut sv = grpc::ServerBuilder::new_plain();
    sv.http.set_port(s.port);
    sv.add_service(chat_grpc::ChatServer::new_service_def(handler));
    sv.http.set_cpu_pool_threads(4);
    let server = sv.build().expect("server");

    println!("Chat server started on port {}", s.port);

    while server.is_alive() {
        std::thread::park();
    }
    return Ok(());
}

fn main() -> Result<(), grpc::Error> {
    let opt = Opt::from_args();

    match opt {
        Opt::Serve(s) => serve(&s),
        Opt::Client(c) => client(&c),
    }
}
