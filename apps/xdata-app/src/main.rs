use tokio_util::sync::CancellationToken;
use tonic::transport::Server;

mod greeter;
use greeter::{GreeterServer, MyGreeter};

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let addr = "[::]:8080".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    // Create a cancellation token
    let cancellation_token = CancellationToken::new();
    let token_clone = cancellation_token.clone();

    // Spawn a task to listen for shutdown signals
    tokio::spawn(async move {
        wait_for_shutdown_signal().await;
        println!("Shutdown signal received, cancelling server...");
        token_clone.cancel();
    });

    // Run the server with graceful shutdown using the cancellation token
    let result = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve_with_shutdown(addr, cancellation_token.cancelled())
        .await;

    if let Err(e) = result {
        eprintln!("Server error: {}", e);
    }

    println!("Goodbye, world!");
}

/// k8s uses sigterm to signal shutdown
async fn wait_for_shutdown_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to setup SIGTERM handler");
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to setup SIGINT handler");
    tokio::select! {
        _ = sigterm.recv() => {
            println!("Received SIGTERM, shutting down.");
        }
        _ = sigint.recv() => {
            println!("Received SIGINT, shutting down.");
        }
    }
}
