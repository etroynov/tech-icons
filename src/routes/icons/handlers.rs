use axum::{
    Json,
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::fs;

use crate::utils::file_names_in;

const STATIC_PATH: &'static str = "./assets/icons";
const SVG_CONTENT_TYPE: &'static str = "image/svg+xml";

#[derive(Deserialize)]
pub struct QueryParams {
    i: Option<String>,
}

pub async fn get_icon(Query(params): Query<QueryParams>) -> impl IntoResponse {
    let Some(icons_name_list) = params.i else {
        return (StatusCode::BAD_REQUEST, "You didn't specify any icons!").into_response();
    };

    let path = format!("{}/{}.svg", STATIC_PATH, icons_name_list);

    match fs::read(path).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, SVG_CONTENT_TYPE)], bytes).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Icon file not found").into_response(),
    }
}

pub async fn get_names() -> impl IntoResponse {
    match file_names_in("./assets/icons") {
        Ok(files) => Json(files).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Icon files not found").into_response(),
    }
}
