use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub tool: Tool,
    pub tests: HashMap<String, Test>,
    pub directory: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Tool {
    pub cmd: PathBuf,
    pub ext: String,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub extension: String,
    pub data: String,
    pub encoding: String,
    pub transform: Option<String>,
    pub hex_edit: Option<HexEdit>,
}

#[derive(Debug, Deserialize)]
pub struct HexEdit {
    pub start: String,
    pub end: String,
    pub data: String,
}

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}
