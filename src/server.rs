#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use protos::chat;
use protos::chat_grpc;

use futures::{stream, Future, Sink};

#[derive(Clone)]
struct ChatMessage {
    name: String,
    message: String,
}

impl ChatMessage {
    fn into_sent(self) -> chat::SentMessage {
        let mut m = chat::SentMessage::new();
        m.set_name(self.name);
        m.set_message(self.message);
        return m;
    }
}

#[derive(Clone)]
struct MessageLogWriter {
    locked: Arc<(Mutex<Vec<ChatMessage>>, Condvar)>,
}

impl MessageLogWriter {
    fn new() -> MessageLogWriter {
        let mu = Mutex::new(vec![]);
        let c = Condvar::new();
        MessageLogWriter {
            locked: Arc::new((mu, c)),
        }
    }

    fn reader(&self) -> MessageLogReader {
        return MessageLogReader {
            locked: self.locked.clone(),
            next: 0,
            wait: false,
        };
    }

    fn write(&self, m: ChatMessage) {
        let &(ref lock, ref cvar) = &*self.locked;
        let mut log = lock.lock().unwrap();
        log.push(m);
        cvar.notify_all();
    }
}

#[derive(Clone)]
struct MessageLogReader {
    locked: Arc<(Mutex<Vec<ChatMessage>>, Condvar)>,
    next: usize,
    wait: bool,
}

impl Iterator for MessageLogReader {
    type Item = ChatMessage;

    fn next(&mut self) -> Option<ChatMessage> {
        let &(ref lock, ref cvar) = &*self.locked;
        let mut msgs = lock.lock().unwrap();
        loop {
            if self.next < msgs.len() {
                break;
            }
            msgs = cvar.wait(msgs).unwrap();
        }

        let msg = Some(msgs[self.next].clone());
        self.next += 1;
        return msg;
    }
}

struct ClientMap {
    members: HashMap<u64, String>,
    ids: rand::StdRng,
}

#[derive(Clone)]
pub struct ChatServer {
    // Maps unique ID to name
    clients: Arc<Mutex<ClientMap>>,
    messages: MessageLogWriter,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        let cm = ClientMap {
            members: HashMap::new(),
            ids: rand::StdRng::new().unwrap(),
        };
        return ChatServer {
            clients: Arc::new(Mutex::new(cm)),
            messages: MessageLogWriter::new(),
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
        for _ in 1..20 {
            reply.session = clients.ids.next_u64();
            if !clients.members.contains_key(&reply.session) {
                clients.members.insert(reply.session, req.name);
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
        _ctx: grpcio::RpcContext<'_>,
        req: chat::Registered,
        sink: grpcio::ServerStreamingSink<chat::SentMessage>,
    ) {
        let _name = {
            let clients = self.clients.lock().unwrap();
            match clients.members.get(&req.session) {
                None => {
                    sink.fail(grpcio::RpcStatus::new(
                        grpcio::RpcStatusCode::NotFound,
                        Some("Session id not found".to_owned()),
                    ));
                    return;
                }
                Some(name) => name.clone(),
            }
        };

        let chat_iter = self
            .messages
            .reader()
            .map(|m| (m.into_sent(), Default::default()));
        let s: futures::sink::SendAll<_, _> =
            sink.send_all(stream::iter_ok::<_, grpcio::Error>(chat_iter));

        let f = s
            .map(|_| {})
            .map_err(|e| println!("failed to handle error: {:?}", e));
        std::thread::spawn(|| f.wait());
    }

    fn say(
        &self,
        _ctx: grpcio::RpcContext<'_>,
        req: chat::ChatMessage,
        sink: grpcio::UnarySink<chat::Empty>,
    ) {
        let clients = self.clients.lock().unwrap();
        let name = match clients.members.get(&req.session) {
            None => {
                sink.fail(grpcio::RpcStatus::new(
                    grpcio::RpcStatusCode::NotFound,
                    Some("Session not found".to_owned()),
                ));
                return;
            }
            Some(n) => n,
        };
        println!("{}: {}", name, req.message.clone());
        let cm = ChatMessage {
            name: name.clone(),
            message: req.message.clone(),
        };

        self.messages.write(cm);
        sink.success(chat::Empty::new());
    }
}
