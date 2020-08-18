use myapi::myapi_v1::pb_api_v1;
use prost::Message;
use std::error::Error;
use std::{thread, time};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("0.0.0.0:8080").await.unwrap();
    println!("Connected to 0.0.0.0:8080");

    let (rx, _) = stream.into_split();

    tokio::spawn(async move {
        if let Err(e) = listener_thread(rx).await {
            panic!("error occurred {}", e);
        }
    });

    thread::sleep(time::Duration::from_secs(10));
}

async fn listener_thread(stream: OwnedReadHalf) -> Result<(), Box<dyn Error>> {
    let mut reader = FramedRead::new(stream, BytesCodec::new());

    loop {
        while let Some(message) = reader.next().await {
            match message {
                Ok(bytes) => {
                    let response: pb_api_v1::LoginResponse =
                        Message::decode_length_delimited(bytes).unwrap();
                    println!("username is {}", response.username);
                }
                Err(err) => println!("Socket closed with error: {:?}", err),
            }
        }
    }
}
