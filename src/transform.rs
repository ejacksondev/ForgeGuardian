use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::toml_structs::Tool;

pub fn transform(tool: &Tool, file_path: &Path, transform: &String) -> Result<PathBuf, Box<dyn Error>> {
    let transformed_command = transform.replace("[path]", file_path.to_str().unwrap());
    Command::new(&tool.cmd)
        .args(transformed_command.split_whitespace())
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", tool.cmd.display(), e))?;

    // Ensure file_path has correct extension added (.original.xz)
    let mut ext: String = file_path.extension().unwrap().to_string_lossy().to_string();
    ext.push_str(&tool.ext);
    let mut new_path: PathBuf = file_path.to_path_buf();
    new_path.set_extension(ext);
    Ok(new_path)
}