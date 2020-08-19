use crate::app_protocol::Wrapper;
use bytes::BytesMut;
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
        Ok(Some(wrapper))
    }
}

// impl Encoder for MyCodec {
//     type Item = pb_api_v1::Wrapper;
//     type Error = std::io::Error;

//     fn encode(&mut self, message: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {}
// }
