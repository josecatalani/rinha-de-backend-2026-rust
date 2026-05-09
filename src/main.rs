mod routes;
mod vectorizer;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(routes::health::get))
        .route("/fraud-score", post(routes::fraud_score::post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
