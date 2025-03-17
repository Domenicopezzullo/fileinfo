use std::{env, ffi::OsString, fs::File, path::Path, process};
use chrono::{DateTime, Local};

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

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

    let last_modified = match metadata.modified() {
        Ok(time) => time,
        Err(err) => return Err(format!("Failed to get last modified time for file '{}': {}", file_path, err)),
    };

    let datetime: DateTime<Local> = last_modified.into();
    let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let filetype = if metadata.is_dir() { "Folder" } else { "File" }.to_string();

    Ok((size, formatted_time, filetype, is_sym, file_name.to_owned()))
}

#[cfg(windows)]
fn print_windows_specific_metadata(metadata: &std::fs::Metadata) -> Result<(), String> {
    let file_attributes = metadata.file_attributes();
    let creation_time = metadata.creation_time();
    let last_access_time = metadata.last_access_time();
    let last_write_time = metadata.last_write_time();
    let volume_serial_number = metadata.volume_serial_number();
    let number_of_links = metadata.number_of_links();
    let file_index = metadata.file_index();
    let change_time = metadata.change_time();

    println!("Windows-specific metadata:");
    println!("  - File Attributes: 0x{:x}", file_attributes);
    println!("  - Creation Time: {}", creation_time);
    println!("  - Last Access Time: {}", last_access_time);
    println!("  - Last Write Time: {}", last_write_time);
    println!("  - Volume Serial Number: {:?}", volume_serial_number);
    println!("  - Number of Links: {:?}", number_of_links);
    println!("  - File Index: {:?}", file_index);
    println!("  - Change Time: {:?}", change_time);

    Ok(())
}

fn format_size(size: u64) -> (f64, &'static str) {
    if size >= 1_000_000_000 {
        (size as f64 / 1_000_000_000.0, "GB")
    } else if size >= 1_000_000 {
        (size as f64 / 1_000_000.0, "MB")
    } else if size >= 1_000 {
        (size as f64 / 1_000.0, "KB")
    } else {
        (size as f64, "bytes")
    }
}

fn main_with_windows_metadata() {
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

            #[cfg(windows)]
            {
                let file = File::open(file_path).map_err(|err| format!("Failed to open file '{}': {}", file_path, err))?;
                let metadata = file.metadata().map_err(|err| format!("Failed to get metadata for file '{}': {}", file_path, err))?;
                if let Err(e) = print_windows_specific_metadata(&metadata) {
                    eprintln!("ERROR: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}

