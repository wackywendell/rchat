fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/protos",
        includes: &[],
        input: &["src/protos/chat.proto"],
        rust_protobuf: true,
        ..Default::default()
    }).expect("protoc-rust-grpc");
}
