*A client/server example for Tokio*

Build a basic client and server using [Tokio](https://github.com/tokio-rs/tokio) to illustrate the following concepts:

1. spawning a thread to process incoming messages
2. splitting TcpStream into separate send and receive
3. encoding/decoding protobuf messages using the [Prost library](https://github.com/danburkert/prost**


**To Run***

```shell
#start the server first
cargo run --bin server

#in another terminal
cargo run --bin client
```
