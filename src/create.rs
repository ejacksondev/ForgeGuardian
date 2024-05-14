use crate::Test;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create(file_path: &Path, test: &Test) -> Result<(), Box<dyn Error>> {
    // Support b64 or utf8 only
    let decoded_data = match test.encoding.to_lowercase().as_str() {
        "b64" => BASE64_STANDARD
            .decode(&test.data)
            .map_err(|e| format!("Failed to decode base64 data: {}", e))?,
        _ => test.data.as_bytes().to_vec(), // No encoding or unknown encoding
    };
    fs::write(file_path, decoded_data).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}
