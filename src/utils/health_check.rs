use tokio::net::TcpListener;
use log::{info, error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_tcp_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;

    info!("Server running on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // Handle each connection in a separate task.
            while let Ok(n) = socket.read(&mut buf).await {
                if n == 0 { break; } // Connection was closed
            }
        });
    }
}