use std::error::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Server is starting...");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server running on 127.0.0.1:8080");
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            let mut buffer = vec![0;1024];

            loop {
                let bytes_read = match socket.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Connection closed by: {}", addr );
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                //echo data back to client
                if let Err(e) = socket.write_all(&buffer[..bytes_read]).await {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });

    }
}
