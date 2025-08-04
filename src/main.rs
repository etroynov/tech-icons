use axum::{Router, routing::get};

async fn root() -> &'static str {
    "Tech icons"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("App started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
