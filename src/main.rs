use std::{env, fs::File, os::unix::fs::MetadataExt, process};
use chrono::{DateTime, Local};


fn main() {
    let file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: {} <file_path>", env::args().next().unwrap());
            process::exit(1);
        }
    };

    let (size, last_modified, filetype) = process(file_path);

    let (size_value, size_suffix) = if size >= 1_000_000_000 {
        (size as f64 / 1_000_000_000.0, "gigabytes")
    } else if size >= 1_000_000 {
        (size as f64 / 1_000_000.0, "megabytes")
    } else {
        (size as f64, "bytes")
    };


    println!("\nType: {}\nSize: {} {}\nLast Modified: {}", &filetype, size_value, &size_suffix, &last_modified);
}

fn process(file_path: String) -> (u64, String, String) {
    let file = File::open(&file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to open file '{}': {}", file_path, &err);
        process::exit(1);
    });

    let metadata = file.metadata().unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to get metadata for file '{}': {}", file_path, &err);
        process::exit(1);
    });

    let size = metadata.size();


    let last_modified = metadata.modified().unwrap_or_else(|err| {
        eprintln!("ERROR: Failed to get last modified time for file '{}': {}", file_path, &err);
        process::exit(1);
    });
    let datetime: DateTime<Local> = last_modified.into();
    let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    
    let mut filetype = String::new();


    match metadata.is_dir() {
        true => filetype.push_str("Folder"),
        false => filetype.push_str("File"),
    }


    (size, formatted_time, filetype)
}
