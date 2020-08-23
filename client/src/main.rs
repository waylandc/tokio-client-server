use myapi::app_protocol::*;
use myapi::codec::MyCodec;
use protobuf::Message;
use std::convert::From;
use std::error::Error;
use std::str::from_utf8;
use std::{thread, time};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::stream::StreamExt;
use tokio_util::codec::FramedRead;

static CLIENT_VERSION: &'static str = "1.0";

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("0.0.0.0:8080").await.unwrap();
    println!("Connected to 0.0.0.0:8080");

    let (rx, tx) = stream.into_split();

    tokio::spawn(async move {
        if let Err(e) = do_handshake(tx, rx).await {
            panic!("error occurred {}", e);
        }
    });
    //TODO if we dont sleep here, execution finishes before
    //spawned thread has time to do anything
    //eventually we will spin another thread to take user input and send
    //requests to server on _tx channel
    thread::sleep(time::Duration::from_secs(10));
}

//do_handshake sends the client version to the server to initialize
//and verify the protocol version. An Error is returned if the current
//version is not supported
async fn do_handshake(
    mut snd: OwnedWriteHalf,
    mut recv: OwnedReadHalf,
) -> Result<String, Box<dyn Error>> {
    //to handshake with server, we sent our client version
    //immediately after connecting
    let buf: Vec<u8> = CLIENT_VERSION.as_bytes().to_vec();
    snd.write_all(&buf).await.unwrap();

    let mut in_buf: [u8; 256] = [0; 256];
    let n = recv
        .read(&mut in_buf)
        .await
        .expect("Failed to receive handshake");

    if n == 0 {
        return Err("server failed to return handshake".into());
    } else {
        let server_protocol = from_utf8(&in_buf).unwrap().to_string();

        println!("server is running protocol version {}", server_protocol);
        let ver: String = CLIENT_VERSION.into();
        //TODO enumerate our version somewhere
        //Check that our protocol version matches server's
        let proto_check = server_protocol.trim_matches(char::from(0)).eq(&ver);
        println!(
            "{:?} {} {}",
            server_protocol.trim_matches(char::from(0)),
            ver.len(),
            proto_check
        );
        match proto_check {
            true => println!(
                "Handshake successful, using protocol version {}",
                &server_protocol
            ),
            _ => return Err("Client version unsupported".into()),
        }

        // Handshake successful, proceed
        let mut reader = FramedRead::new(recv, MyCodec::new());

        let mut lr = LoginRequest::default();
        lr.set_username("wayc".to_string());
        lr.set_password("mysecretp@ssw0rd".to_string());

        let mut wrapper = Wrapper::default();
        wrapper.set_api("1.0".to_string());
        wrapper.set_loginReq(lr.clone());

        let response_buf: Vec<u8> = wrapper.write_to_bytes().unwrap();
        let buf_size: u8 = buf.len() as u8;
        println!("calculated msg length is {}", buf_size);

        let nn = buf_size as u16;
        let num = nn.to_be_bytes(); //[u8, u8]

        let mut concat: Vec<u8> = num.to_vec();
        for &s in &response_buf {
            concat.push(s);
        }

        println!("sending request {} bytes long", concat.len());
        snd.write_all(&concat).await.unwrap();

        while let Some(message) = reader.next().await {
            match message {
                Ok(msg) => println!(
                    "received login response: {:?}",
                    msg.get_loginResp().get_status()
                ),
                Err(_) => continue,
            }
        }
        Ok("OK".to_string())
    }
}
