use myapi::app_protocol::{LoginResponse, Wrapper};

use protobuf::Message;
use std::convert::From;
use std::error::Error;
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;

static PROTOCOL_VERSION: &'static str = "1.0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server bound and listening on 0.0.0.0:8080");

    loop {
        let (mut stream, addr) = listener.accept().await?;

        //do handshake
        let mut buf: [u8; 1024] = [0; 1024];
        let n = stream
            .read(&mut buf)
            .await
            .expect("failed to read handshake");

        if n == 0 {
            return Err("client failed handshake".into());
        }

        let client_version = from_utf8(&buf).unwrap().to_string();
        let ver: String = PROTOCOL_VERSION.into();

        let proto_check = client_version.trim_matches(char::from(0)).eq(&ver);
        println!(
            "protocheck is {} {} {}",
            client_version.len(),
            ver.len(),
            proto_check
        );
        match proto_check {
            true => {
                println!("client version OK");
                stream.write_all(PROTOCOL_VERSION.as_bytes()).await.unwrap();
            }
            _ => {
                stream.write_all(b"UNSUPPORTED_VERSION").await.unwrap();
                return Err("Client is using unsupported version".into());
            }
        };

        let (rx, tx) = stream.into_split();
        println!("client connected from {}", addr);
        tokio::spawn(async move {
            let ret = process(tx, rx).await;
            match ret {
                Ok(t) => println!("process returned {}", t),
                Err(e) => println!("process returned error: {}", e),
            }
        });
    }
}

async fn process(
    mut snd: OwnedWriteHalf,
    mut _recv: OwnedReadHalf,
) -> Result<bool, Box<dyn Error>> {
    //TODO just hardcoding a pb to send across wire for now
    // We will just encode a protobuf message and then send to client

    let mut lr: myapi::app_protocol::LoginReponse = LoginResponse::default();
    lr.set_username("Satoshi".to_string());
    lr.set_status(true);

    let mut wrapper: Wrapper = Wrapper::default();
    wrapper.set_api("1.0".to_string());
    wrapper.set_loginResp(lr.clone());

    let response_buf: Vec<u8> = wrapper.write_to_bytes().unwrap();
    let buf_size: u8 = response_buf.len() as u8;

    let nn: u8 = buf_size as u8;
    let num = nn.to_be_bytes();

    let mut concat: Vec<u8> = num.to_vec();
    for &s in &response_buf {
        concat.push(s);
    }

    println!("sending LoginResponse {} bytes long", concat.len());
    snd.write_all(&concat).await.unwrap();
    println!(
        "Sent response to {:?}",
        wrapper.get_loginResp().get_username()
    );
    Ok(true)
}
