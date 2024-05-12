use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn mutate(start: usize, end: usize, data: String, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let bytes = fs::read(file_path).expect("Failed to read file");
    let new_bytes = [&bytes[0..start], &data.as_bytes(), &bytes[end..bytes.len()]].concat();

    overwrite_file(&file_path, new_bytes).expect("TODO: panic message");

    // TODO doc - specify inclusive range and incl example. Specify removal of bytes as "". Specify hex syntax only

    Ok(())
}

fn overwrite_file(file_path: &Path, contents: Vec<u8>) -> Result<(), std::io::Error> {
    let temp_path = format!("{}.tmp", file_path.display());

    let mut temp_file = File::create(&temp_path)?;
    temp_file.write_all(contents.as_ref())?;

    fs::rename(&temp_path, file_path)?;

    Ok(())
}