use tokio::net::TcpListener;
use log::{info, error};

pub async fn start_tcp_health_check_server() {
    let addr = "0.0.0.0:8080";
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Health check TCP server listening on {}", addr);
            loop {
                match listener.accept().await {
                    Ok((_socket, _)) => {
                        // Connection established, can log or handle specifics here
                    }
                    Err(e) => error!("Failed to accept connection: {}", e),
                }
            }
        }
        Err(e) => error!("Failed to bind TCP health check server: {}", e),
    }
}
