pub fn clean_file_name(name: String) -> String {
    name.strip_suffix(".svg")
        .unwrap_or(&name)
        .trim_end_matches("_light")
        .trim_end_matches("_dark")
        .to_string()
}
