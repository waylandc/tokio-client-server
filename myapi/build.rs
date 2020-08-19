extern crate protoc_rust;
//use protoc_rust::Customize;

// Build script to compile protobuf files into Rust using prost_build tool
fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["src/app_protocol.proto"])
        .include("src")
        .run()
        .expect("protoc");
}
