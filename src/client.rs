use futures::Stream;

use protos::chat;
use protos::chat_grpc;
use protos::chat_grpc::Chat;

pub struct ChatClient {
    id: u64,
    cli: chat_grpc::ChatClient,
}

impl ChatClient {
    pub fn new(session: u64, client: chat_grpc::ChatClient) -> ChatClient {
        ChatClient {
            id: session,
            cli: client,
        }
    }

    pub fn register(
        client: chat_grpc::ChatClient,
        name: String,
    ) -> Result<ChatClient, grpc::Error> {
        let mut r = chat::Registration::new();
        r.name = name;

        match client.register(grpc::RequestOptions::new(), r).wait() {
            Err(e) => Err(e),
            Ok((_, v, _)) => Ok(ChatClient::new(v.session, client)),
        }
    }

    pub fn say(&self, msg: String) -> Result<(), grpc::Error> {
        let mut cm = chat::ChatMessage::new();
        cm.session = self.id;
        cm.message = msg.trim().to_owned();
        return self
            .cli
            .say(grpc::RequestOptions::new(), cm)
            .wait()
            .map(|_| ());
    }

    pub fn listen(&self) -> impl Stream<Item = chat::SentMessage, Error = grpc::Error> {
        let mut r = chat::Registered::new();
        r.set_session(self.id);
        let stream = self.cli.listen(grpc::RequestOptions::new(), r);
        return stream.drop_metadata();
    }
}
