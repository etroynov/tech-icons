use std::{cmp::min, fmt::Write};

const CELL: i32 = 300;
const MARGIN: i32 = 44;
const ONE_ICON: f32 = 48.0;
const SCALE: f32 = ONE_ICON / (CELL as f32 - MARGIN as f32);

pub fn get_svg_sprite(icons: Vec<Vec<u8>, per_line: usize) -> String {
    let per_line = per_line.max(1);
    let icon_svg_list: Vec<&str> = icon_names
        .iter()
        .filter_map(|name| icons.get(*name).map(|s| s.as_str()))
        .collect();

    let length =
        (min(per_line * CELL as usize, icon_names.len() * CELL as usize) as i32 - MARGIN).max(0);
    let height = if icon_svg_list.is_empty() {
        0
    } else {
        ((icon_svg_list.len() as f32 / per_line as f32).ceil() as i32 * CELL - MARGIN).max(0)
    };

    let scaled_width = length as f32 * SCALE;
    let scaled_height = height as f32 * SCALE;

    let mut out = String::with_capacity(1024 + icon_svg_list.len() * 128);

    let _ = write!(
        out,
        r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" fill="none" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1">"#,
        scaled_width, scaled_height, length, height
    );

    for (index, svg) in icon_svg_list.iter().enumerate() {
        let x = (index % per_line) as i32 * CELL;
        let y = (index / per_line) as i32 * CELL;
        let _ = write!(out, r#"<g transform="translate({}, {})">{}</g>"#, x, y, svg);
    }

    out.push_str("</svg>");
    out
}
