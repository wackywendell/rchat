#![warn(rust_2018_idioms)]

use rand::{Rng, StdRng};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

use grpc::{Error, RequestOptions, SingleResponse, StreamingResponse};

use protos::chat::ChatMessage as ProtoMessage;
use protos::chat::{Empty, Registered, Registration, SentMessage};
use protos::chat_grpc::Chat;

#[derive(Clone)]
struct ChatMessage {
    name: String,
    message: String,
}

impl ChatMessage {
    fn into_sent(self) -> SentMessage {
        let mut m = SentMessage::new();
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
    ids: StdRng,
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
            ids: StdRng::new().unwrap(),
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

impl Chat for ChatServer {
    fn register(&self, _m: RequestOptions, req: Registration) -> SingleResponse<Registered> {
        println!("Registering {}", req.name);
        let mut reply = Registered::new();
        let mut clients = self.clients.lock().unwrap();
        for _ in 1..20 {
            reply.session = clients.ids.next_u64();
            match clients.members.entry(reply.session) {
                Entry::Vacant(v) => {
                    v.insert(req.name.clone());
                    return SingleResponse::completed(reply);
                }
                Entry::Occupied(_) => continue,
            }
        }

        SingleResponse::err(Error::Other("Ran out of sessions"))
    }

    fn listen(&self, _m: RequestOptions, req: Registered) -> StreamingResponse<SentMessage> {
        let _name = {
            let clients = self.clients.lock().unwrap();
            match clients.members.get(&req.session) {
                None => {
                    return StreamingResponse::err(Error::Other("Session id not found"));
                }
                Some(name) => name.clone(),
            }
        };

        let chat_iter = self.messages.reader().map(|m| m.into_sent());
        StreamingResponse::iter(chat_iter)
    }

    fn say(&self, _m: RequestOptions, req: ProtoMessage) -> SingleResponse<Empty> {
        let clients = self.clients.lock().unwrap();
        let name = match clients.members.get(&req.session) {
            None => {
                return SingleResponse::err(Error::Other("Session id not found"));
            }
            Some(n) => n,
        };
        println!("{}: {}", name, req.message.clone());
        let cm = ChatMessage {
            name: name.clone(),
            message: req.message.clone(),
        };

        self.messages.write(cm);
        SingleResponse::completed(Empty::new())
    }
}
