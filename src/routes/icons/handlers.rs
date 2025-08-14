use std::collections::HashSet;

use axum::{
    extract::Query,
    http::{StatusCode, header},
    response::IntoResponse,
};
use futures::future::join_all;
use serde::Deserialize;
use tokio::fs;

use crate::utils::get_svg_sprite;

const ICONS_PER_LINE: i32 = 15;
const STATIC_PATH: &str = "./assets/icons";
const SVG_CONTENT_TYPE: &str = "image/svg+xml";

#[derive(Deserialize)]
pub struct QueryParams {
    i: Option<String>,
    theme: Option<String>,
}

fn is_safe_filename(name: &str) -> bool {
    !name.contains("..")
        && !name.contains('/')
        && !name.contains('\\')
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

async fn load_icon(name: &str, theme: &str) -> Option<Vec<u8>> {
    let themed = format!("{}/{}_{}.svg", STATIC_PATH, name, theme);
    let base = format!("{}/{}.svg", STATIC_PATH, name);

    match fs::read(&themed).await {
        Ok(data) => Some(data),
        Err(_) => fs::read(&base).await.ok(),
    }
}

pub async fn get_icon(Query(params): Query<QueryParams>) -> impl IntoResponse {
    let Some(icons_name_list) = params.i else {
        return (StatusCode::BAD_REQUEST, "You didn't specify any icons!").into_response();
    };

    let theme = params.theme.unwrap_or_else(|| "dark".to_string());

    let names: HashSet<&str> = icons_name_list
        .split(',')
        .map(|name| name.trim())
        .filter(|name| !name.is_empty() && is_safe_filename(name))
        .collect();

    let futures = names.into_iter().map(|name| load_icon(name, &theme));
    let results = join_all(futures).await;

    let icons: Vec<Vec<u8>> = results.into_iter().flatten().collect();

    if icons.is_empty() {
        return (StatusCode::NOT_FOUND, "No icons found").into_response();
    }

    let svg_sprite = get_svg_sprite(icons, ICONS_PER_LINE);
    ([(header::CONTENT_TYPE, SVG_CONTENT_TYPE)], svg_sprite).into_response()
}
