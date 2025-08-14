use std::fmt::Write;

const CELL: i32 = 300;
const MARGIN: i32 = 44;
const ONE_ICON: f32 = 48.0;
const SCALE: f32 = ONE_ICON / (CELL as f32 - MARGIN as f32);

pub fn get_svg_sprite(icons: Vec<Vec<u8>>, per_line: i32) -> String {
    let per_line = per_line.max(1);

    let length = (per_line * CELL - MARGIN).max(0);
    let height = if icons.is_empty() {
        0
    } else {
        ((icons.len() as f32 / per_line as f32).ceil() as i32 * CELL - MARGIN).max(0)
    };

    let scaled_w = (length as f32 * SCALE) as i32;
    let scaled_h = (height as f32 * SCALE) as i32;

    let mut out = String::new();
    write!(
        out,
        r#"<svg width="{scaled_w}" height="{scaled_h}" viewBox="0 0 {length} {height}" xmlns="http://www.w3.org/2000/svg">"#
    )
    .unwrap();

    for (idx, icon) in icons.iter().enumerate() {
        let col = idx as i32 % per_line;
        let row = idx as i32 / per_line;
        let x = col * CELL;
        let y = row * CELL;

        let icon_s = String::from_utf8_lossy(icon)
            .trim_start_matches(|c: char| c != '<')
            .trim_end_matches(|c: char| c != '>')
            .to_string();

        write!(out, r#"<g transform="translate({x}, {y})">{icon_s}</g>"#).unwrap();
    }

    out.push_str("</svg>");
    out
}
