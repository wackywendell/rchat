#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use rand::Rng;
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};

use protos::chat;
use protos::chat_grpc;

use crossbeam::channel;
use futures::{Future, Sink};
use structopt::StructOpt;

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

#[derive(Clone)]
struct ChatMessage {
    name: String,
    message: String,
}

struct ServerClient {
    name: String,
    send: channel::Sender<ChatMessage>,
    receive: channel::Receiver<ChatMessage>,
}

struct ClientMap {
    members: HashMap<u64, ServerClient>,
    ids: rand::StdRng,
}

#[derive(Clone)]
struct ChatServer {
    // Maps unique ID to name
    clients: Arc<Mutex<ClientMap>>,
}

impl ChatServer {
    fn new() -> ChatServer {
        let cm = ClientMap {
            members: HashMap::new(),
            ids: rand::StdRng::new().unwrap(),
        };
        return ChatServer {
            clients: Arc::new(Mutex::new(cm)),
        };
    }
}

impl chat_grpc::Serve for ChatServer {
    fn register(
        &self,
        _ctx: grpcio::RpcContext<'_>,
        req: chat::Registration,
        sink: grpcio::UnarySink<chat::Registered>,
    ) {
        println!("Registering {}", req.name);
        let mut reply = chat::Registered::new();
        let mut clients = self.clients.lock().unwrap();
        let (tx, rx) = channel::unbounded();
        let client = ServerClient {
            name: req.name,
            send: tx,
            receive: rx,
        };
        for _ in 1..20 {
            reply.session = clients.ids.next_u64();
            if !clients.members.contains_key(&reply.session) {
                clients.members.insert(reply.session, client);
                sink.success(reply);
                return;
            }
        }
        sink.fail(grpcio::RpcStatus::new(
            grpcio::RpcStatusCode::ResourceExhausted,
            Some("Ran out of sessions".to_owned()),
        ));
    }

    fn listen(
        &self,
        ctx: grpcio::RpcContext<'_>,
        req: chat::Registered,
        sink: grpcio::ServerStreamingSink<chat::SentMessage>,
    ) {
        let rx = {
            let mut clients = self.clients.lock().unwrap();
            match clients.members.get(&req.session) {
                None => {
                    sink.fail(grpcio::RpcStatus::new(
                        grpcio::RpcStatusCode::NotFound,
                        Some("Session id not found".to_owned()),
                    ));
                    return;
                }
                Some(s) => s.receive.clone(),
            }
        };

        let f = |sink|{
            match rx.recv() {
                Some(s) => {
                    let mut msg = chat::SentMessage::new();
                    msg.set_message(s.message);
                    msg.set_name(s.name);
                    // sink.send consumes the sink, returning it when its finished.

                    let f = sink
                        .send((msg, Default::default()))
                        .map(|_| ())
                        .map_err(|_| ());
                    ctx.spawn(f);
                }
                None => break,
            }
        }
    }

    fn say(
        &self,
        _ctx: grpcio::RpcContext<'_>,
        req: chat::ChatMessage,
        sink: grpcio::UnarySink<chat::Empty>,
    ) {
        let clients = self.clients.lock().unwrap();
        let member = match clients.members.get(&req.session) {
            None => {
                sink.fail(grpcio::RpcStatus::new(
                    grpcio::RpcStatusCode::NotFound,
                    Some("Session not found".to_owned()),
                ));
                return;
            }
            Some(n) => n,
        };
        let cm = ChatMessage {
            name: member.name.clone(),
            message: req.message,
        };
        for (&id, member) in &clients.members {
            if id == req.session {
                continue;
            }
            member.send.send(cm.clone());
        }
        println!("   <{}> {}", cm.name, cm.message);
        sink.success(chat::Empty::new());
    }
}

fn serve(s: &ServeOpt) -> Result<(), grpcio::Error> {
    let env = Arc::new(grpcio::Environment::new(2));
    let instance = ChatServer::new();
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

struct ChatClient {
    id: u64,
    cli: chat_grpc::ServeClient,
}

impl ChatClient {
    fn new(session: u64, client: chat_grpc::ServeClient) -> ChatClient {
        ChatClient {
            id: session,
            cli: client,
        }
    }

    fn register(client: chat_grpc::ServeClient, name: String) -> Result<ChatClient, grpcio::Error> {
        let mut r = chat::Registration::new();
        r.name = name;
        let s = client.register(&r)?;

        Ok(ChatClient::new(s.session, client))
    }

    fn say(&self, msg: String) -> Result<(), grpcio::Error> {
        let mut cm = chat::ChatMessage::new();
        cm.session = self.id;
        cm.message = msg.trim().to_owned();
        return self.cli.say(&cm).map(|_| ());
    }
}

fn client(o: &ClientOpt) -> Result<(), grpcio::Error> {
    let env = Arc::new(grpcio::Environment::new(2));
    let cb = grpcio::ChannelBuilder::new(env);
    let addr = format!("{}:{}", "127.0.0.1", o.port);
    let ch = cb.connect(&addr);
    let c = chat_grpc::ServeClient::new(ch);
    let cli = ChatClient::register(c, o.name.clone())?;

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
