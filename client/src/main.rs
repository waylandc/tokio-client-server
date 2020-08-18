use bytes::Bytes;
use myapi::myapi_v1;
use myapi::myapi_v1::pb_api_v1;
use prost::Message;
use std::error::Error;
use std::io::{Cursor};
use std::{thread, time};
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::TcpStream;
use tokio::prelude::*;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::convert::From;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("0.0.0.0:8080").await.unwrap();
    println!("Connected to 0.0.0.0:8080");

    let (rx, _) = stream.into_split();

    tokio::spawn(async move {
        if let Err(e) = process(rx).await {
            panic!("error occurred {}", e);
        }
    });

    thread::sleep(time::Duration::from_secs(10));
}

async fn process(mut stream: OwnedReadHalf) -> Result<(), Box<dyn Error>> {

    loop {
        // fixed size buffer to read stream into
        let mut buf = vec![0; 1024];

        let n = stream
            .read(&mut buf)
            .await
            .expect("failed to read from socket");
        println!("n is {}", n);

        if n == 0 {
            println!("read nothing");
            return Err("read nothing".into());
        } else {
            //clone the bytes read into a properly sized buffer
            let xx = Vec::from_iter(buf[0..n].iter().cloned());
            println!("length of xx {}", xx.len());
            let y = Cursor::new(&xx);
            let msg: pb_api_v1::LoginResponse = Message::decode_length_delimited(y).unwrap();

            println!("{:?}", msg.username);
            //let b = Bytes::from(buf);
            //println!("received {:?}", str::from_utf8(&b).unwrap()); 
        }
    }
}

