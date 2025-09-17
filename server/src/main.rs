mod configuration;

use std::net::SocketAddr;
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum::http::HeaderMap;
use validator::Validate;
use crate::configuration::{Config, EmailRequest};

#[tokio::main]
async fn main() {
    let config = match configuration::load_config("config.yaml") {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to load config {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/validate", post(validate_email))
        .with_state(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let socket = match  tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to bind to {}", e);
            std::process::exit(1);
        }
    };

    println!("Server listening on {}", addr);
    match axum::serve(
        socket,
        app.into_make_service()
    ).await {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to serve {}", e);
            std::process::exit(1);
        }
    }
}

async fn validate_email(
    State(config): State<Config>,
    headers: HeaderMap,
    Json(payload): Json<EmailRequest>,
) -> impl IntoResponse {
    // Check auth header
    if !headers.contains_key("authorization") {
        return (StatusCode::UNAUTHORIZED, "no_token").into_response();
    }

    let auth_header = headers["authorization"].to_str().unwrap_or("invalid_token").to_string();
    if auth_header != format!("Bearer {}", config.token) {
        return (StatusCode::UNAUTHORIZED, "invalid_token").into_response();
    }

    // Invalid mail
    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Invalid email format").into_response();
    }

    // Email is explicitly allowed
    if config.allowed_mails.iter().any(|d| d.to_lowercase() == payload.email) {
        return (StatusCode::OK, "email_allowed").into_response();
    }

    // Email domain is allowed
    let email_domain = payload.email.split('@').last().unwrap_or("").to_lowercase();
    let domain_allowed = config.allowed_domains.iter()
        .any(|d| d.to_lowercase() == email_domain);
    if domain_allowed {
        return (StatusCode::OK, "domain_allowed").into_response();
    }

    (StatusCode::FORBIDDEN, "not_allowed").into_response()
}
