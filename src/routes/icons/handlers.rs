use std::fmt::format;

use axum::{
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
};
use futures::future::join_all;
use serde::Deserialize;
use tokio::fs;
use tracing::info;

use crate::utils::get_svg_sprite;

const ICONS_PER_LINE: i32 = 15;
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

    let paths: Vec<&str> = icons_name_list.split(',').collect();
    let futures = paths
        .into_iter()
        .map(|p| async move { fs::read(format!("{}/{}.svg", STATIC_PATH, p)).await });
    let results = join_all(futures).await;

    let icons: Vec<Vec<u8>> = results.into_iter().filter_map(|r| r.ok()).collect();

    let svg_sprite = get_svg_sprite(icons, ICONS_PER_LINE);

    ([(header::CONTENT_TYPE, SVG_CONTENT_TYPE)], svg_sprite).into_response()
}
