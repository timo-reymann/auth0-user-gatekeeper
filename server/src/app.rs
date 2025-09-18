use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use validator::Validate;
use crate::configuration::{Config, EmailRequest};

pub fn create_app(config: Config) -> Router {
    Router::new()
        .route("/isAllowed", post(check_allowed_by_mail))
        .with_state(config)
}

async fn check_allowed_by_mail(
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
    if config.allowed_mails.contains(&payload.email) {
        return (StatusCode::OK, "email_allowed").into_response();
    }

    // Email domain is allowed
    let email_domain = payload.email.split('@').last().unwrap_or("").to_lowercase();
    let domain_allowed = config.allowed_domains.contains(&email_domain);
    if domain_allowed {
        return (StatusCode::OK, "domain_allowed").into_response();
    }

    (StatusCode::FORBIDDEN, "not_allowed").into_response()
}
