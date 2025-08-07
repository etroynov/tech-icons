mod middlewares;
mod routes;
mod utils;

use axum::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let app = Router::new()
        .nest("/icons", routes::icons::routes())
        .layer(middlewares::logger::trace_layer());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("App started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
