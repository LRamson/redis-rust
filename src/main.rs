use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("[LOG] Running on Port 6379...");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("[LOG] New Connection!");

        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}

async fn process_socket(mut socket: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                return;
            }
            Ok(_n) => {
                if let Err(e) = socket.write_all(b"+PONG\r\n").await {
                    println!("Erro ao enviar resposta: {}", e);
                    return;
                }
            }
            Err(e) => {
                println!("Erro na leitura do socket: {}", e);
                return;
            }
        }
    }
}