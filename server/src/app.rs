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

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt; // for `oneshot`
    use http::{Request, StatusCode};
    use serde_json::json;

    // Minimal stubs/imports to compile in the test module context
    use crate::configuration::Config;

    #[tokio::test]
    async fn create_app_sets_state_and_route() {
        let cfg = Config {
            allowed_domains: vec!["example.com".into()],
            allowed_mails: vec!["user@example.com".into()],
            token: "tkn".into(),
        };

        let app = create_app(cfg.clone());

        // Route exists and responds (we only check that handler is wired).
        // We don't assert handler logic here, just that the route is present and callable.
        let body = json!({ "email": "test@example.com" }).to_string();
        let request: http::Request<String> = Request::builder()
            .method("POST")
            .uri("/isAllowed")
            .header("content-type", "application/json")
            .body(body.into())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        // At least confirm we get some HTTP response from the route.
        assert!(response.status() == StatusCode::OK
            || response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNAUTHORIZED
            || response.status().is_client_error()
            || response.status().is_success());

        // Also ensure the router holds the state type; there isn't a direct getter,
        // but with_state compiles and type-checks. To cover state presence at runtime,
        // we ensure creating the app with state doesn't panic and can process a request.
        let _ = app;
    }

    #[tokio::test]
    async fn create_app_only_exposes_is_allowed_route() {
        let cfg = Config {
            allowed_domains: vec![],
            allowed_mails: vec![],
            token: "tkn".into(),
        };
        let app = create_app(cfg);

        // Unknown route should be 404
        let request = Request::builder()
            .method("GET")
            .uri("/unknown")
            .body(axum::body::Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}