use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::utils::file_names_in;

pub async fn get_names() -> impl IntoResponse {
    match file_names_in("./assets/icons") {
        Ok(files) => Json(files).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Icon files not found").into_response(),
    }
}
