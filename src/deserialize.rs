use crate::toml_structs::*;
use std::fs;
use std::path::PathBuf;

pub fn deserialize(args: Cli) -> ConfigToml {
    let toml_str = fs::read_to_string(&args.path).expect("Failed to read file");
    let mut config_toml: ConfigToml =
        toml::from_str(&toml_str).expect("Failed to deserialize Cargo.toml");

    let binding = PathBuf::from(&args.path);
    let config_dir = binding
        .parent()
        .expect("Failed to get parent directory of config file");
    config_toml.directory = Option::from(PathBuf::from(config_dir));

    config_toml
}
