use crate::app_protocol::Wrapper;
use bytes::{Buf, BytesMut};
use protobuf;
use std::io::Error;
use tokio_util::codec::Decoder;

pub struct MyCodec;

impl MyCodec {
    pub fn new() -> MyCodec {
        MyCodec {}
    }
}

impl Decoder for MyCodec {
    type Item = Wrapper;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let wrapper: Wrapper = protobuf::parse_from_bytes(&buf).unwrap();
        if !buf.is_empty() {
            let len = buf.len();
            buf.advance(len);
            Ok(Some(wrapper))
        } else {
            Ok(None)
        }
    }
}

// impl Encoder for MyCodec {
//     type Item = pb_api_v1::Wrapper;
//     type Error = std::io::Error;

//     fn encode(&mut self, message: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {}
// }
