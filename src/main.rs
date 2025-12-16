#![allow(unused_imports)]
use std::{{
    io::{Read, Write},
    net::TcpListener}};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();
                println!("request: {}", String::from_utf8_lossy(&buffer[..]));
                stream.write_all(b"+PONG\r\n").unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
