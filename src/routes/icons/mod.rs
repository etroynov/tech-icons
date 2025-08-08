pub mod handlers;

use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_icon))
        .route("/names", get(handlers::get_names))
}
