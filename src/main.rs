use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod resp;
use resp::{Value, RespHandler};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("[LOG] Listening on 127.0.0.1:6379");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            process_connection(stream).await;
        });
    }
}

async fn process_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 512];
        let bytes_read = match stream.read(&mut buffer).await {
            Ok(0) => return, 
            Ok(n) => n,
            Err(_) => return,
        };

        let mut handler = RespHandler {
            buffer: buffer[..bytes_read].to_vec(),
            cursor: 0
        };

        if let Some(value) = handler.parse() {
            if let Value::Array(args) = value {
                if let Some(Value::BulkString(command)) = args.get(0) {
                    process_command(command, &args, &mut stream).await;
                }
            }
        } else {
            println!("Error parsing command");
        }
    }
}


async fn process_command(command: &str, args: &Vec<Value>, stream: &mut TcpStream) {
    match command.to_uppercase().as_str() {
        "PING" => {
            let _ = stream.write_all(b"+PONG\r\n").await;
        }
        "ECHO" => {
            if let Some(Value::BulkString(arg)) = args.get(1) {
                let response = Value::BulkString(arg.clone()).serialize();
                let _ = stream.write_all(response.as_bytes()).await;
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}