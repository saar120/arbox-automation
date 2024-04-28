use tokio::net::TcpListener;
use log::{info, error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_tcp_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    info!("Server running on {}", addr);

    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    while let Ok(n) = socket.read(&mut buf).await {
                        if n == 0 { break; } // Connection was closed
                    }
                });
                info!("Health check executed: Connection accepted");
            },
            Err(e) => {
                error!("Failed to accept connection: {}", e);
                break;
            }
        }
    }

    Ok(())
}