use myapi::codec::MyCodec;
use std::error::Error;
use std::{thread, time};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use tokio::stream::StreamExt;
use tokio_util::codec::FramedRead;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("0.0.0.0:8080").await.unwrap();
    println!("Connected to 0.0.0.0:8080");

    let (rx, _tx) = stream.into_split();

    tokio::spawn(async move {
        if let Err(e) = listener_thread(rx).await {
            panic!("error occurred {}", e);
        }
    });

    //TODO if we dont sleep here, execution finishes before
    //spawned thread has time to do anything
    //eventually we will spin another thread to take user input and send
    //requests to server on _tx channel
    thread::sleep(time::Duration::from_secs(10));
}

async fn listener_thread(stream: OwnedReadHalf) -> Result<(), Box<dyn Error>> {
    let mut reader = FramedRead::new(stream, MyCodec::new());

    while let message = reader.next().await {
        match message {
            Some(msg) => println!("received: {:?}", msg),
            None => continue,
        }
    }
    Ok(())
}
