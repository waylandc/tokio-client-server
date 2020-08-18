//use tokio::prelude::*;
use myapi::myapi_v1::pb_api_v1;
use prost::Message;
use std::error::Error as StdError;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

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

async fn _process(mut stream: TcpStream) {
    stream.write_all(b"\x00\x01Hello world").await.unwrap();
    stream.flush();
}

async fn process(mut stream: TcpStream) {
    // We will just encode a protobuf message and then send to client
    let response: pb_api_v1::LoginResponse = pb_api_v1::LoginResponse {
        api: "1.0".to_string(),
        status: true,
        username: "Satoshi".to_string(),
    };
    let mut buf = Vec::new();
    //let mut buf = vec![0; 1024];
    //encoded.reserve(response.encoded_len());
    //response.encode(&mut encoded).unwrap();
    match Message::encode_length_delimited(&response, &mut buf) {
        Ok(m) => m,
        Err(e) => println!("encode error: {}", e),
    };
    println!("encoded: {:?}", buf);
    stream.write_all(&buf).await.unwrap();
}
