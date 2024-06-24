extern crate prost_build;
extern crate protobuf_src;

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    prost_build::compile_protos(
        &[
            "src/proto/cometGate.proto",
            "src/proto/cometLogin.proto",
            "src/proto/cometScene.proto",
        ],
        &["src/"],
    )
    .unwrap();
}
