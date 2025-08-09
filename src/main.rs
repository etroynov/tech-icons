mod middlewares;
mod routes;
mod utils;

use axum::Router;
use tokio::signal::unix::{SignalKind, signal};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let app = Router::new()
        .nest("/icons", routes::icons::routes())
        .nest("/api", routes::api::routes())
        .layer(middlewares::logger::trace_layer());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("App started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let mut sig_int = signal(SignalKind::interrupt()).expect("cannot catch SIGINT");
            let mut sig_term = signal(SignalKind::terminate()).expect("cannot catch SIGTERM");

            tokio::select! {
                _ = sig_int.recv() => {},
                _ = sig_term.recv() => {},
            }

            eprintln!("Shutdown signal received, stopping...");
        })
        .await
        .unwrap();
}
