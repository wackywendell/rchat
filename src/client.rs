#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use protos::chat;
use protos::chat_grpc;

pub struct ChatClient {
    id: u64,
    cli: chat_grpc::ServeClient,
}

impl ChatClient {
    pub fn new(session: u64, client: chat_grpc::ServeClient) -> ChatClient {
        ChatClient {
            id: session,
            cli: client,
        }
    }

    pub fn register(
        client: chat_grpc::ServeClient,
        name: String,
    ) -> Result<ChatClient, grpcio::Error> {
        let mut r = chat::Registration::new();
        r.name = name;
        let s = client.register(&r)?;

        Ok(ChatClient::new(s.session, client))
    }

    pub fn say(&self, msg: String) -> Result<(), grpcio::Error> {
        let mut cm = chat::ChatMessage::new();
        cm.session = self.id;
        cm.message = msg.trim().to_owned();
        return self.cli.say(&cm).map(|_| ());
    }
}
