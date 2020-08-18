use prost::{DecodeError, Message};
use std::option::Option;
use std::string::String;
use std::io::Cursor;
//use bytes::{Buf, Bytes};
pub mod pb_api_v1 {
    include!("./api_v1.rs");
}

// create_wrapper is used to create the outer wrapper, used when parsing inbound messages
pub fn create_wrapper() -> pb_api_v1::Wrapper {
    pb_api_v1::Wrapper::default()
}

pub fn myparse_message(message: Vec<u8>) -> Result<pb_api_v1::Wrapper, DecodeError> {
    let b = Cursor::new(message);
    Message::decode(b)
}
// parse_message decodes a buffer and returns our Wrapper protobuf message
pub fn parse_message<T: bytes::Buf>(message: T) -> Result<pb_api_v1::Wrapper, DecodeError> {
    Message::decode(message)
}

//TODO should we embed the send in these methods? Passing around Wrappers
//doesn't seem very dev friendly
pub fn create_login_request(user: String, passwd: String) -> pb_api_v1::Wrapper {
    let w = pb_api_v1::Wrapper {
        api: String::from("1.0"),
        msg: Option::Some(pb_api_v1::wrapper::Msg::LoginReq(pb_api_v1::LoginRequest {
            api: String::from("1.0"),
            username: user,
            password: passwd,
        })),
    };
    println!("loginrequest: {:?}", &w);
    w
}

pub fn create_login_response(
    ok: bool,
    u: &str,
    api: &str,
) -> pb_api_v1::Wrapper {
    pb_api_v1::Wrapper {
        api: api.to_string(),
        msg: Option::Some(pb_api_v1::wrapper::Msg::LoginResp(
            pb_api_v1::LoginResponse {
                api: api.to_string(),
                status: ok,
                username: u.to_string(),
            },
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::myapi_v1::create_login_request;
    use crate::myapi_v1::pb_api_v1;
    use prost::Message;
    #[test]
    fn test_encode_login() {
        let w: pb_api_v1::Wrapper =
            create_login_request(String::from("waylandc"), String::from("badpassw0rd"));
        let mut buf = Vec::with_capacity(1024);
        match Message::encode(&w, &mut buf) {
            Ok(m) => m,
            Err(error) => println!("encode error: {}", error),
        }
        let mut buf = &*buf;
        let xx: pb_api_v1::Wrapper = match Message::decode(&mut buf) {
            Ok(m) => m,
            Err(_) => panic!("problem"),
        };
        println!("encode/decode {:?}", xx.msg);

        match xx.msg {
            Some(pb_api_v1::wrapper::Msg::LoginReq(lr)) => {
                println!("password is {}", lr.password);
                assert_eq!(lr.password, String::from("badpassw0rd"));
            }
            _ => println!("nothing"),
        }
    }

    #[test]
    fn test_decode() {
        let resp = pb_api_v1::LoginResponse {
            api: "1.0".to_string(),
            status: true,
            username: "me".to_string(),
        };
        let mut buf = vec![0; 512];

        match Message::encode(&resp, &mut buf) {
            Ok(m) => m,
            Err(error) => println!("encode error: {}", error),
        }
        println!("encode {:?}", resp.status);

    }
}
