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


// interface

pub trait Chat {
    fn register(&self, o: ::grpc::RequestOptions, p: super::chat::Registration) -> ::grpc::SingleResponse<super::chat::Registered>;

    fn listen(&self, o: ::grpc::RequestOptions, p: super::chat::Registered) -> ::grpc::StreamingResponse<super::chat::SentMessage>;

    fn say(&self, o: ::grpc::RequestOptions, p: super::chat::ChatMessage) -> ::grpc::SingleResponse<super::chat::Empty>;
}

// client

pub struct ChatClient {
    grpc_client: ::grpc::Client,
    method_Register: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::chat::Registration, super::chat::Registered>>,
    method_Listen: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::chat::Registered, super::chat::SentMessage>>,
    method_Say: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::chat::ChatMessage, super::chat::Empty>>,
}

impl ChatClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ChatClient {
            grpc_client: grpc_client,
            method_Register: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Chat/Register".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Listen: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Chat/Listen".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Say: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Chat/Say".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ChatClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ChatClient::with_client(c)
        })
    }
}

impl Chat for ChatClient {
    fn register(&self, o: ::grpc::RequestOptions, p: super::chat::Registration) -> ::grpc::SingleResponse<super::chat::Registered> {
        self.grpc_client.call_unary(o, p, self.method_Register.clone())
    }

    fn listen(&self, o: ::grpc::RequestOptions, p: super::chat::Registered) -> ::grpc::StreamingResponse<super::chat::SentMessage> {
        self.grpc_client.call_server_streaming(o, p, self.method_Listen.clone())
    }

    fn say(&self, o: ::grpc::RequestOptions, p: super::chat::ChatMessage) -> ::grpc::SingleResponse<super::chat::Empty> {
        self.grpc_client.call_unary(o, p, self.method_Say.clone())
    }
}

// server

pub struct ChatServer;


impl ChatServer {
    pub fn new_service_def<H : Chat + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Chat",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Chat/Register".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.register(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Chat/Listen".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.listen(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Chat/Say".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.say(o, p))
                    },
                ),
            ],
        )
    }
}
