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

const METHOD_SERVE_REGISTER: ::grpcio::Method<super::chat::Registration, super::chat::Registered> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Serve/Register",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ServeClient {
    client: ::grpcio::Client,
}

impl ServeClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ServeClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn register_opt(&self, req: &super::chat::Registration, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::chat::Registered> {
        self.client.unary_call(&METHOD_SERVE_REGISTER, req, opt)
    }

    pub fn register(&self, req: &super::chat::Registration) -> ::grpcio::Result<super::chat::Registered> {
        self.register_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_async_opt(&self, req: &super::chat::Registration, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chat::Registered>> {
        self.client.unary_call_async(&METHOD_SERVE_REGISTER, req, opt)
    }

    pub fn register_async(&self, req: &super::chat::Registration) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::chat::Registered>> {
        self.register_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Serve {
    fn register(&self, ctx: ::grpcio::RpcContext, req: super::chat::Registration, sink: ::grpcio::UnarySink<super::chat::Registered>);
}

pub fn create_serve<S: Serve + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVE_REGISTER, move |ctx, req, resp| {
        instance.register(ctx, req, resp)
    });
    builder.build()
}
