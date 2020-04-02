extern crate fortress_bake;

use fortress_bake::{
    app::StatusOr,
    bake::{
        self,
        InputOutput,
    },
    file,
};
use std::path::PathBuf;

fn copy_font(input_output: &InputOutput) -> StatusOr<()> {
    let directory_iterator = input_output.input.fonts.read_dir()
        .map_err(|e| format!("Bad font dir: {:?}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name());

    for base_name in directory_iterator {
        if let Some(base_name_str) = base_name.to_str() {
            if !base_name_str.ends_with(".ttf") {
                continue;
            }

            let input = input_output.input.fonts.join(&base_name);
            let output = input_output.output.fonts.join(&base_name);
            file::util::copy_file(input, output)?;
        }
    }

    Ok(())
}

fn main() -> StatusOr<()> {
    let mut root = PathBuf::new();
    root.push("..");
    let input_output = InputOutput::new(root)?;

    copy_font(&input_output)?;

    bake::run(input_output)
}