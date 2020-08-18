use std::env;
extern crate prost_build;

// Build script to compile protobuf files into Rust using prost_build tool
fn main() {
    env::set_var("OUT_DIR", "src/");
    prost_build::compile_protos(&["src/protobuf_api/api-v1.proto"], &["src/"]).unwrap();
}

// Include the `items` module, which is generated from items.proto.
// pub mod items {
//     include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
// }

// pub fn create_large_shirt(color: String) -> items::Shirt {
//     let mut shirt = items::Shirt::default();
//     shirt.color = color;
//     shirt.set_size(items::shirt::Size::Large);
//     shirt
// }
