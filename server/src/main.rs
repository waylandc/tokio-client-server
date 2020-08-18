//use tokio::prelude::*;
use std::error::Error as StdError;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
use myapi::myapi_v1::pb_api_v1;
use prost::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server bound and listening on 0.0.0.0:8080");

    loop {
        let (stream, _) = listener.accept().await?;

        println!("client connected");
        tokio::spawn(async move {
            process(stream).await;
        });
    }
}


async fn process(mut stream: TcpStream) {
    stream.write_all(b"\x00\x01Hello world").await.unwrap();
    stream.flush();
}

async fn _process(mut stream: TcpStream) {
    // We will just encode a protobuf message and then send to client
    let response: pb_api_v1::LoginResponse = pb_api_v1::LoginResponse {
        api: "1.0".to_string(),
        status: true,
        username: "Satoshi".to_string(),
    };
    let mut encoded = Vec::new();
    encoded.reserve(response.encoded_len());
    response.encode(&mut encoded).unwrap();
    stream.write_all(&encoded).await.unwrap();

}

