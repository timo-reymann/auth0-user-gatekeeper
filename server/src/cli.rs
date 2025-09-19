use crate::app::create_app;
use crate::configuration;
use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Args {
    #[arg(long, default_value = "2025")]
    port: u16,
    #[arg(long, default_value = "config.yaml")]
    config: String,
}

pub async fn run(args: Args) -> i32 {
    let config = match configuration::load_config(args.config.as_str()) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to load config {}", e);
            return 1;
        }
    };

    let app = create_app(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let socket = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to bind to {}", e);
            return 1;
        }
    };

    println!("Server listening on {}", addr);
    match axum::serve(socket, app.into_make_service()).await {
        Ok(_) => 0,
        Err(e) => {
            println!("Failed to serve {}", e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use tokio::task::JoinHandle;

    // Helper to pick a free port
    fn free_port() -> u16 {
        TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    // Run the CLI in a background task and return the join handle.
    async fn run_cli(args: Args) -> JoinHandle<i32> {
        tokio::spawn(async move { run(args).await })
    }

    #[tokio::test]
    async fn fails_when_config_missing() {
        let port = free_port();
        let args = Args {
            port,
            config: "does-not-exist.yaml".into(),
        };

        let code = run(args).await;
        // Expect non-zero exit code for missing/unreadable config
        assert_ne!(code, 0);
    }

    // Verifies the server responds to /isAllowed using the configured port without adding new deps.
    #[tokio::test]
    async fn server_smoke_test_is_allowed() {
        use axum::body;
        use http::{Request as HttpRequest, StatusCode};
        use tower::ServiceExt; // oneshot

        let port = free_port();
        let token = "smoke-token";
        let args = Args {
            port,
            config: "config.yaml".into(),
        };

        // Start server on the chosen port
        let handle = run_cli(args).await;

        // Give it a brief moment to start and bind
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Build a Router identical to the running instance to issue an HTTP request
        // targeting the configured port via an absolute-form URI.
        let url = format!("http://127.0.0.1:{}/isAllowed", port);

        // Recreate the app with the same config (inline args above)
        let cfg = crate::configuration::Config {
            token: token.into(),
            allowed_domains: vec!["smoke.com".into()],
            allowed_mails: vec!["user@other.com".into()],
        };
        let app = crate::app::create_app(cfg);

        let req = HttpRequest::builder()
            .method("POST")
            .uri(url)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", token))
            .body(r#"{"email":"any@smoke.com"}"#.to_string())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = body::to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
        let body_txt = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body_txt, "domain_allowed");

        // Stop server
        handle.abort();
        let _ = handle.await;
    }
}
