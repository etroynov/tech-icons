use std::collections::HashMap;

const CELL: usize = 300;
const MARGIN_FIX: i32 = 44;

pub fn get_svg_icon(
    icon_names: &[&str],
    per_line: usize,
    icons: &HashMap<String, String>,
    scale: f32,
) {
    let svg_icons: Vec<&str> = icon_names
        .iter()
        .filter_map(|name| icons.get(*name).map(|s| s.as_str()))
        .collect();
}
