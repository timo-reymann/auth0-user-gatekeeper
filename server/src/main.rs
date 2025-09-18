mod app;
mod configuration;

use crate::app::create_app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = match configuration::load_config("config.yaml") {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to load config {}", e);
            std::process::exit(1);
        }
    };

    let app = create_app(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let socket = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to bind to {}", e);
            std::process::exit(1);
        }
    };

    println!("Server listening on {}", addr);
    match axum::serve(socket, app.into_make_service()).await {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to serve {}", e);
            std::process::exit(1);
        }
    }
}
