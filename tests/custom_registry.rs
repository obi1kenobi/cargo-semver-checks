use assert_cmd::Command;
use axum::Router;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 5000;

#[tokio::test]
async fn test_custom_registry() {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    tokio::join!(
        async {
            tokio::time::timeout(Duration::from_secs(10), async {
                while reqwest::get(format!("http://{HOST}:{PORT}")).await.is_err() {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                tokio::task::spawn_blocking(move || verify_custom_registry(token))
                    .await
                    .expect("registry failure");
            })
            .await
            .expect("timeout failure");
        },
        serve(using_serve_dir(), cloned_token)
    );
}

async fn serve(app: Router, token: CancellationToken) {
    let addr = SocketAddr::from_str(&format!("{HOST}:{PORT}")).expect("Failed to parse HOST:PORT");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    axum::serve(
        listener,
        app.layer(TimeoutLayer::new(Duration::from_secs(10))),
    )
    .with_graceful_shutdown(shutdown_signal(token))
    .await
    .expect("failed to serve");
}

fn using_serve_dir() -> Router {
    Router::new().nest_service("/", ServeDir::new("test_crates/custom_registry/registry"))
}

fn verify_custom_registry(token: CancellationToken) {
    let mut cmd =
        Command::cargo_bin("cargo-semver-checks").expect("failed to execute cargo-semver-checks");
    let result = cmd
        .current_dir("test_crates/custom_registry/new")
        .args(["semver-checks", "--registry", "custom-registry"])
        .assert();
    token.cancel();
    result.success();
}

async fn shutdown_signal(token: CancellationToken) {
    tokio::select! {
        _ = token.cancelled() => {}
    }
}
