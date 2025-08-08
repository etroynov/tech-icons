use std::fs;
use std::io;
use std::path::Path;

pub fn file_names_in(dir: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let mut names = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let md = entry.metadata()?;

        if md.is_file() {
            match entry.file_name().to_str() {
                Some(name) => names.push(name.to_string()),
                None => println!("Error with file"),
            }
        }
    }

    Ok(names)
}
