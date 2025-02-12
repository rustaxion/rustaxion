#![allow(unused)]

use std::{cmp::Ordering, fmt::Write, fs};

extern crate anyhow;
extern crate prost_build;
extern crate protobuf_src;

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let proto = prost_build::compile_protos(
        &[
            "lib/proto/cometGate.proto",
            "lib/proto/cometLogin.proto",
            "lib/proto/cometScene.proto",
        ],
        &["lib/proto/"],
    );

    match proto {
        Ok(_) => {}
        Err(err) => {
            let inner = err.into_inner().unwrap();
            eprintln!("Failed to compile proto files: {}", inner);
            std::process::exit(1);
        }
    };
}
