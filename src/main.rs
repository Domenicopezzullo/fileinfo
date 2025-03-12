use std::{env, ffi::OsString, fs::File, os::unix::fs::MetadataExt, path::Path, process};
use chrono::{DateTime, Local};

fn main() {
    let file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <file_path>", env::args().next().unwrap());
            process::exit(1);
        }
    };

    match process(&file_path) {
        Ok((size, last_modified, filetype, is_sym, file_name)) => {
            let (size_value, size_suffix) = format_size(size);
            println!("\n\nName: {}\nType: {}\nSize: {} {}\nLast Modified: {}\nIs a symlink: {}", &file_name.to_string_lossy(), &filetype, &size_value, &size_suffix, &last_modified, &is_sym);
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}

fn process(file_path: &str) -> Result<(u64, String, String, bool, OsString), String> {
    let file = File::open(file_path).map_err(|err| format!("Failed to open file '{}': {}", file_path, err))?;
    let metadata = file.metadata().map_err(|err| format!("Failed to get metadata for file '{}': {}", file_path, err))?;
    let file_name = Path::new(file_path).file_name().expect("File path could not be resolved!");

    let mut is_sym = false;


    if metadata.is_symlink() {
        is_sym = true;
    } 


    let size = metadata.size();
    let last_modified = metadata.modified().map_err(|err| format!("Failed to get last modified time for file '{}': {}", file_path, err))?;
    let datetime: DateTime<Local> = last_modified.into();
    let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let filetype = if metadata.is_dir() { "Folder" } else { "File" }.to_string();

    Ok((size, formatted_time, filetype, is_sym, file_name.to_owned()))
}

fn format_size(size: u64) -> (f64, &'static str) {
    if size >= 1_000_000_000 {
        (size as f64 / 1_000_000_000.0, "gigabytes")
    } else if size >= 1_000_000 {
        (size as f64 / 1_000_000.0, "megabytes")
    } else {
        (size as f64, "bytes")
    }
}
