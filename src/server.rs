#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use rand::Rng;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use protos::chat;
use protos::chat_grpc;

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
        m
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
        MessageLogReader {
            locked: self.locked.clone(),
            next: 0,
            wait: false,
        }
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
        msg
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
        ChatServer {
            clients: Arc::new(Mutex::new(cm)),
            messages: MessageLogWriter::new(),
        }
    }
}

impl Default for ChatServer {
    fn default() -> Self {
        ChatServer::new()
    }
}

impl chat_grpc::Chat for ChatServer {
    fn register(
        &self,
        _m: grpc::RequestOptions,
        req: chat::Registration,
    ) -> grpc::SingleResponse<chat::Registered> {
        println!("Registering {}", req.name);
        let mut reply = chat::Registered::new();
        let mut clients = self.clients.lock().unwrap();
        for _ in 1..20 {
            reply.session = clients.ids.next_u64();
            match clients.members.entry(reply.session) {
                Entry::Vacant(v) => {
                    v.insert(req.name.clone());
                    return grpc::SingleResponse::completed(reply);
                }
                Entry::Occupied(_) => continue,
            }
        }

        grpc::SingleResponse::err(grpc::Error::Other("Ran out of sessions"))
    }

    fn listen(
        &self,
        _m: grpc::RequestOptions,
        req: chat::Registered,
    ) -> grpc::StreamingResponse<chat::SentMessage> {
        let _name = {
            let clients = self.clients.lock().unwrap();
            match clients.members.get(&req.session) {
                None => {
                    return grpc::StreamingResponse::err(grpc::Error::Other("Session id not found"));
                }
                Some(name) => name.clone(),
            }
        };

        let chat_iter = self.messages.reader().map(|m| m.into_sent());
        grpc::StreamingResponse::iter(chat_iter)
    }

    fn say(
        &self,
        _m: grpc::RequestOptions,
        req: chat::ChatMessage,
    ) -> grpc::SingleResponse<chat::Empty> {
        let clients = self.clients.lock().unwrap();
        let name = match clients.members.get(&req.session) {
            None => {
                return grpc::SingleResponse::err(grpc::Error::Other("Session id not found"));
            }
            Some(n) => n,
        };
        println!("{}: {}", name, req.message.clone());
        let cm = ChatMessage {
            name: name.clone(),
            message: req.message.clone(),
        };

        self.messages.write(cm);
        grpc::SingleResponse::completed(chat::Empty::new())
    }
}
