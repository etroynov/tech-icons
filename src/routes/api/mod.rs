pub mod handlers;

use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/icons", get(handlers::get_names))
}
