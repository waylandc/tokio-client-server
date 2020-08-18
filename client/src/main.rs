use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use tokio::prelude::*;
use bytes::Bytes;
use std::error::Error;

//use myapi::myapi_v1;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("0.0.0.0:8080").await.unwrap();
    println!("Connected to 0.0.0.0:8080");

    let (rx, _) = stream.into_split();
/*
    loop {
        let mut buf = vec![0; 1024];
        let n = rx.read(&mut buf[..]).await.unwrap();
        println!("n is {}", n);
        if n == 0 {
            println!("read nothing");
            return;
        } else {
            println!("received {:?}", Bytes::from(buf));
        }
    }
*/
    tokio::spawn(async move {
        if let Err(e) = listener(rx).await {
            panic!("error occurred {}", e);
        }
    });
}

async fn listener(mut stream: OwnedReadHalf) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf[..]).await.expect("failed to read from socket");
        println!("n is {}", n);
        if n == 0 {
            println!("read nothing");
            return Err("read nothing".into());
        } else {
            println!("received {:?}", Bytes::from(buf));
        }
    }
}

