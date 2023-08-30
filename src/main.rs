use std::str::FromStr;

use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, routing::post, BoxError, Json, Router,
};
use serde_json::Value;
use tower::ServiceBuilder;
use tower_http::decompression::RequestDecompressionLayer;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", post(root)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled server error")
            }))
            .layer(RequestDecompressionLayer::new()),
    );

    let addr = std::net::SocketAddr::from_str(&format!("{}:{}", "127.0.0.1", 8000)).unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(Json(value): Json<Value>) -> Json<Value> {
    Json(value)
}
