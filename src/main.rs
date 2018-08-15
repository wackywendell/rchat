#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use futures::{future, Stream};
use grpc::{Error, ServerBuilder};
use structopt::StructOpt;

use protos::chat_grpc::ChatClient as GrpcClient;
use protos::chat_grpc::ChatServer as GrpcServer;

use crate::client::ChatClient;
use crate::server::ChatServer;

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

fn client(o: &ClientOpt) -> Result<(), Error> {
    let client = GrpcClient::new_plain("::1", o.port, Default::default())?;
    let cli = ChatClient::register(client, o.name.clone())?;

    let listener = cli.listen();

    let listen_stream = listener
        .map_err(|e| println!("Error listening: {}", e))
        .map(move |m| {
            println!("{}: {}", m.name, m.message);
            Ok(())
        });

    let buffed = std::io::BufReader::new(tokio::io::stdin());
    let line_by_line = tokio::io::lines(buffed);
    let drop_errs = line_by_line.map_err(|e| println!("error reading: {}", e));
    let read_stream = drop_errs.map(move |l| {
        match cli.say(&l) {
            Ok(_) => (),
            Err(e) => println!("error saying: {}", e),
        };
        Ok(())
    });

    //let merged = listen_stream.select(read_stream).fold((), |(), v| v);
    let merged = future::lazy(|| {
        tokio::spawn(listen_stream.fold((), |(), v: Result<(), ()>| v));
        tokio::spawn(read_stream.fold((), |(), v: Result<(), ()>| v));
        future::empty::<(), ()>()
    });

    tokio::run(merged);
    Ok(())
}

fn serve(s: &ServeOpt) -> Result<(), Error> {
    let handler = ChatServer::new();
    let mut sv = ServerBuilder::new_plain();
    sv.http.set_port(s.port);
    sv.add_service(GrpcServer::new_service_def(handler));
    sv.http.set_cpu_pool_threads(4);
    let server = sv.build().expect("server");

    println!("Chat server started on port {}", s.port);

    while server.is_alive() {
        std::thread::park();
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    match opt {
        Opt::Serve(s) => serve(&s),
        Opt::Client(c) => client(&c),
    }
}
