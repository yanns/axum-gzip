use std::str::FromStr;

use axum::{routing::post, Json, Router};
use serde_json::Value;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

#[tokio::main]
async fn main() {
    let other_router = make_other_router();
    let app: Router = Router::new()
        .route("/", post(root))
        .merge(other_router)
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        );

    let addr = std::net::SocketAddr::from_str(&format!("{}:{}", "127.0.0.1", 8000)).unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn make_other_router() -> Router<()> {
    Router::new().route("/test", post(root))
}

async fn root(Json(value): Json<Value>) -> Json<Value> {
    Json(value)
}
