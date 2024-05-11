use rrec::{candidate::get_candidate, DemoResult};
use tracing::info;

#[tokio::main]
async fn main() -> DemoResult<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let candidate = get_candidate();
    info!("{}", candidate);

    let app = rrec::router::router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3009")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
