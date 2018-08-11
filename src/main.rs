#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use std::io::Read;
use std::sync::Arc;

use protos::chat_grpc;

use futures::Future;
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

fn client(o: &ClientOpt) -> Result<(), grpcio::Error> {
    let env = Arc::new(grpcio::Environment::new(2));
    let cb = grpcio::ChannelBuilder::new(env);
    let addr = format!("{}:{}", "127.0.0.1", o.port);
    let ch = cb.connect(&addr);
    let c = chat_grpc::ServeClient::new(ch);
    let cli = client::ChatClient::register(c, o.name.clone())?;

    let mut input = String::new();

    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("- {}", input);
                cli.say(input.clone())?;
            }
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
    }

    return Ok(());
}

fn serve(s: &ServeOpt) -> Result<(), grpcio::Error> {
    let env = Arc::new(grpcio::Environment::new(2));
    let instance = server::ChatServer::new();
    let service = chat_grpc::create_serve(instance);
    let mut server = grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", s.port)
        .build()?;
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = futures::oneshot();
    std::thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = std::io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    match rx.wait() {
        Ok(()) => {}
        Err(c) => println!("Err: {}", c),
    }
    server.shutdown().wait()
}

fn main() -> Result<(), grpcio::Error> {
    let opt = Opt::from_args();

    match opt {
        Opt::Serve(s) => {
            println!("Serving {:?}", s);
            serve(&s)
        }
        Opt::Client(c) => {
            println!("Client {:?}", c);
            client(&c)
        }
    }
}
