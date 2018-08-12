// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CHAT_REGISTER: ::grpcio::Method<super::chat::Registration, super::chat::Registered> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Chat/Register",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CHAT_LISTEN: ::grpcio::Method<super::chat::Registered, super::chat::SentMessage> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/Chat/Listen",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CHAT_SAY: ::grpcio::Method<super::chat::ChatMessage, super::chat::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Chat/Say",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ChatClient {
    client: ::grpcio::Client,
}

impl ChatClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ChatClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn register_opt(&self, req: &super::chat::Registration, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::chat::Registered> {
        self.client.unary_call(&METHOD_CHAT_REGISTER, req, opt)
    }

    pub fn register(&self, req: &super::chat::Registration) -> ::grpcio::Result<super::chat::Registered> {
        self.register_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_async_opt(&self, req: &super::chat::Registration, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chat::Registered>> {
        self.client.unary_call_async(&METHOD_CHAT_REGISTER, req, opt)
    }

    pub fn register_async(&self, req: &super::chat::Registration) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chat::Registered>> {
        self.register_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn listen_opt(&self, req: &super::chat::Registered, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::chat::SentMessage>> {
        self.client.server_streaming(&METHOD_CHAT_LISTEN, req, opt)
    }

    pub fn listen(&self, req: &super::chat::Registered) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::chat::SentMessage>> {
        self.listen_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_opt(&self, req: &super::chat::ChatMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::chat::Empty> {
        self.client.unary_call(&METHOD_CHAT_SAY, req, opt)
    }

    pub fn say(&self, req: &super::chat::ChatMessage) -> ::grpcio::Result<super::chat::Empty> {
        self.say_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_async_opt(&self, req: &super::chat::ChatMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chat::Empty>> {
        self.client.unary_call_async(&METHOD_CHAT_SAY, req, opt)
    }

    pub fn say_async(&self, req: &super::chat::ChatMessage) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chat::Empty>> {
        self.say_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Chat {
    fn register(&self, ctx: ::grpcio::RpcContext, req: super::chat::Registration, sink: ::grpcio::UnarySink<super::chat::Registered>);
    fn listen(&self, ctx: ::grpcio::RpcContext, req: super::chat::Registered, sink: ::grpcio::ServerStreamingSink<super::chat::SentMessage>);
    fn say(&self, ctx: ::grpcio::RpcContext, req: super::chat::ChatMessage, sink: ::grpcio::UnarySink<super::chat::Empty>);
}

pub fn create_chat<S: Chat + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAT_REGISTER, move |ctx, req, resp| {
        instance.register(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_CHAT_LISTEN, move |ctx, req, resp| {
        instance.listen(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CHAT_SAY, move |ctx, req, resp| {
        instance.say(ctx, req, resp)
    });
    builder.build()
}
