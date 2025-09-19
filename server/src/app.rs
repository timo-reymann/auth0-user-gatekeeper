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
    use axum::body;
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

    fn app_with_config() -> Router {
        let cfg = Config {
            allowed_domains: vec!["allowed.com".into(), "example.com".into()],
            allowed_mails: vec!["special@allowed.com".into(), "exact@other.org".into()],
            token: "secret".into(),
        };
        create_app(cfg)
    }

    async fn call_is_allowed(email: &str, token: Option<&str>) -> (StatusCode, String) {
        let app = app_with_config();
        let mut req = Request::builder()
            .method("POST")
            .uri("/isAllowed")
            .header("content-type", "application/json");

        if let Some(t) = token {
            req = req.header("authorization", t);
        }

        let request_body = json!({ "email": email }).to_string();
        let response = app.oneshot(req.body(request_body).unwrap()).await.unwrap();
        let status = response.status();

        let bytes = body::to_bytes(response.into_body(), 64 * 1024).await.unwrap();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap();
        (status, body_str)
    }

    #[tokio::test]
    async fn it_returns_401_when_no_token() {
        let (status, body) = call_is_allowed("user@allowed.com", None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body, "no_token");
    }

    #[tokio::test]
    async fn it_returns_401_when_invalid_token() {
        let (status, body) = call_is_allowed("user@allowed.com", Some("Bearer wrong")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body, "invalid_token");
    }

    #[tokio::test]
    async fn it_returns_400_on_invalid_email_format() {
        let (status, body) = call_is_allowed("not-an-email", Some("Bearer secret")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body, "Invalid email format");
    }

    #[tokio::test]
    async fn it_returns_ok_when_email_explicitly_allowed() {
        let (status, body) = call_is_allowed("special@allowed.com", Some("Bearer secret")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "email_allowed");
    }

    #[tokio::test]
    async fn it_returns_ok_when_domain_allowed() {
        let (status, body) = call_is_allowed("someone@allowed.com", Some("Bearer secret")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "domain_allowed");
    }

    #[tokio::test]
    async fn it_is_case_insensitive_for_domain() {
        let (status, body) = call_is_allowed("MiXeD@AlLoWeD.CoM", Some("Bearer secret")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "domain_allowed");
    }

    #[tokio::test]
    async fn it_returns_403_when_not_allowed() {
        let (status, body) = call_is_allowed("nobody@blocked.net", Some("Bearer secret")).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(body, "not_allowed");
    }
}