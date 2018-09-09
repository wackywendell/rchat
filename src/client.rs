use futures::Stream;
use grpc::{Error, RequestOptions};

use protos::chat::{ChatMessage, Registered, Registration, SentMessage};
use protos::chat_grpc::Chat;
use protos::chat_grpc::ChatClient as GrpcClient;

pub struct ChatClient {
    id: u64,
    cli: GrpcClient,
}

impl ChatClient {
    pub fn new(session: u64, client: GrpcClient) -> ChatClient {
        ChatClient {
            id: session,
            cli: client,
        }
    }

    pub fn register(client: GrpcClient, name: String) -> Result<ChatClient, Error> {
        let mut r = Registration::new();
        r.name = name;

        match client.register(RequestOptions::new(), r).wait() {
            Err(e) => Err(e),
            Ok((_, v, _)) => Ok(ChatClient::new(v.session, client)),
        }
    }

    pub fn say(&self, msg: &str) -> Result<(), Error> {
        let mut cm = ChatMessage::new();
        cm.session = self.id;
        cm.message = msg.trim().to_owned();
        self.cli.say(RequestOptions::new(), cm).wait().map(|_| ())
    }

    pub fn listen(&self) -> impl Stream<Item = SentMessage, Error = Error> {
        let mut r = Registered::new();
        r.set_session(self.id);
        let stream = self.cli.listen(RequestOptions::new(), r);
        stream.drop_metadata()
    }
}
