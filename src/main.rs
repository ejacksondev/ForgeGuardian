mod create;
mod transform;
mod mutate;
mod toml_structs;
mod deserialize;

use clap::Parser;
use std::path::{PathBuf};
use crate::create::create;
use crate::deserialize::deserialize;
use crate::toml_structs::*;
use crate::mutate::mutate;
use crate::transform::transform;

/* TODO - can you include support for building files with more complex toolchains? Idk what use there
    would be for this but could need txt - base64 - image - xz - something else idk */
// TODO - Good comprehensive README


fn main() {

    let config = deserialize(Cli::parse());
    // unwrap optional fields if present TODO does this error when not present?
    let config_dir: PathBuf = config.directory.expect("REASON");

    for (test_name, test) in config.tests {

        let mut file_path: PathBuf = config_dir.join(format!("{}{}", test_name, test.extension));

        // Build base file with specified data
        match create(&file_path, &test) {
            Ok(()) => println!("File built successfully: {:?}", file_path),
            Err(err) => eprintln!("Error building file: {}", err),
        }

        // Modify file with specified tool
        // TODO Further work - add option for tool altering ext rather than appending, or for none (?)
        // TODO doc config.tool.cmd can be cmd on PATH or FQPathToBinary
        if let Some(transform_cmd) = &test.transform {
            match transform(&config.tool, &file_path, transform_cmd) {
                Ok(new_file_path) => {
                    file_path = new_file_path;
                    println!("File transformed successfully: {:?}", file_path);
                }
                Err(err) => eprintln!("Error transforming file: {}", err),
            }
        }

        // Alter hex in file where specified
        if let Some(hex_edit) = &test.hex_edit {
            match mutate(hex_edit.start, hex_edit.end, hex_edit.data.clone(), &file_path) {
                Ok(()) => println!("File hex edited successfully: {:?}", file_path),
                Err(err) => eprintln!("Error editing hex of file: {}", err),
            }
        }
    }

}