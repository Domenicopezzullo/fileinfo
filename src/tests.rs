use std::{fs::{metadata, Metadata}, os::unix::fs::MetadataExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = metadata("main.rs")?;
    a.modified()?;

    return Ok(())
}