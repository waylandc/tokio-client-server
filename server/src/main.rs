use myapi::app_protocol::*;

use protobuf::Message;
use std::error::Error as StdError;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server bound and listening on 0.0.0.0:8080");

    loop {
        let (stream, addr) = listener.accept().await?;

        println!("client connected from {}", addr);
        tokio::spawn(async move {
            process(stream).await;
        });
    }
}

async fn process(mut stream: TcpStream) {
    //TODO just hardcoding a pb to send across wire for now
    // We will just encode a protobuf message and then send to client
    let mut lr = LoginResponse::default();
    lr.set_username("Satoshi".to_string());
    lr.set_status(true);

    let mut wrapper = Wrapper::default();
    wrapper.set_api("1.0".to_string());
    wrapper.set_loginResp(lr.clone());

    let buf: Vec<u8> = wrapper.write_to_bytes().unwrap();

    stream.write_all(&buf).await.unwrap();
    println!("Sent {:?}", lr);
}
