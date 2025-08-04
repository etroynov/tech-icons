mod middlewares;

use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
struct QueryParams {
    i: Option<String>,
}

async fn root(Query(params): Query<QueryParams>) -> &'static str {
    if let Some(value) = params.i {
        info!("GET parameter 'i' = {}", value);
    } else {
        println!("GET parameter 'i' not provided");
    }
    "Tech icons"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let app = Router::new()
        .route("/", get(root))
        .layer(middlewares::logger::trace_layer());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("App started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
