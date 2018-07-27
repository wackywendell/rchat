#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use std::io::Read;
use std::sync::Arc;

use protos::chat;
use protos::chat_grpc;

use futures::Future;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct ServeOpt {
    #[structopt(short = "p", long = "port", default_value = "6789")]
    port: u16,
}

#[derive(StructOpt, Debug)]
struct ClientOpt {
    #[structopt(short = "p", long = "port", default_value = "6789")]
    port: u16,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rchat", about = "A simple chat client and server")]
enum Opt {
    #[structopt(name = "serve")]
    Serve(ServeOpt),

    #[structopt(name = "client")]
    Client(ClientOpt),
}

#[derive(Clone)]
struct ChatServer {
    members: Vec<String>,
}

impl chat_grpc::Serve for ChatServer {
    fn register(
        &self,
        _ctx: grpcio::RpcContext<'_>,
        req: chat::Registration,
        sink: grpcio::UnarySink<chat::Registered>,
    ) {
        println!("Registering {}", req.name);
        sink.success(chat::Registered::new());
    }
}

fn serve(s: &ServeOpt) {
    let env = Arc::new(grpcio::Environment::new(2));
    let instance = ChatServer { members: vec![] };
    let service = chat_grpc::create_serve(instance);
    let mut server = grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", s.port)
        .build()
        .unwrap();
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
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Serve(s) => {
            println!("Serving {:?}", s);
            serve(&s);
        }
        c @ Opt::Client { .. } => println!("Client {:?}", c),
    }

    // let server_subparser = clap::SubCommand::with_name("serve").arg(
    //     clap::Arg::with_name("port")
    //         .short("p")
    //         .long("port")
    //         .help("Set port to serve on"),
    // );

    // clap::App::new("rchat")
    //     .version("0.1")
    //     .about("An experimental chat server.")
    //     .author("Wendell Smith")
    //     .subcommand(server_subparser)
    //     .get_matches();

    // println!("Hello, world!");
}
