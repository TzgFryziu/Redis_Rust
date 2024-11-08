#![allow(unused_imports)]

use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage


    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                tokio::spawn(async move {
                    println!("New connection");
                    let mut buffer = [0; 512];
                    loop {
                        let read_count = stream.read(&mut buffer).await.unwrap();
                        if read_count == 0 {
                            println!("Connection ended");
                            break;
                        }
                        stream.write(b"+PONG\r\n").await.unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
