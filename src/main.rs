#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

#[derive(Clone)]
struct ChatServer {
    members: Vec<String>,
}

impl protos::chat_grpc::Serve for ChatServer {
    fn register(
        &self,
        _ctx: grpcio::RpcContext<'_>,
        req: protos::chat::Registration,
        sink: grpcio::UnarySink<protos::chat::Registered>,
    ) {
        println!("Registering {}", req.name);
        sink.success(protos::chat::Registered::new());
    }
}

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
